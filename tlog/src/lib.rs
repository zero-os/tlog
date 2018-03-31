extern crate bincode;
extern crate chrono;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

// Tree
//     fn load: (i/p) namespace
//          -> loop over the backend to load the associated branches metadata in memory
//          -> check if all metadata are legit (if tail was the last node added), update if not. To recover from crashes
//     fn flush: (i/p) pushes all the registered branches metadata to the backend
//     fn scan: (i/p) branch_id (default), start_transaction_id, stop_transaction_id, stop_timestamp -> the whole tree seq t6 -> t5 -> t2 -> t1
//     fn replay: start_transaction_id, stop_transaction_id -> (iterator)
//     fn replayt: start_timestamp, stop_timestamp -> (iterator)

use bincode::{deserialize, serialize};
use chrono::prelude::*;
use std::io::{self, Result};
use std::str;

pub trait Backend {
    /// pushes the data to the backend
    ///
    /// `push` should handle how the data is stored, ie: double linkedlist...etc
    fn push(&mut self, key: &[u8], data: &[u8]) -> Result<()>;

    /// gets the data associated with key provided from the backend
    ///
    /// `fetch` should handle how the data is fetched
    fn fetch(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
}

// TODO: open an issue if the implementation was still as described below
// TODO: for the moment the metadata is updated on every transaction log push (makes 2 request for a single push)
// TODO: update it every a constant number of transaction, every interval of time, or on an event

/// Payload is representation of supported commands
///
/// each command contains its associated data
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Transaction<'a> {
    /// Set <Key> <Value>
    Set((&'a [u8], &'a [u8])),
    /// Delete <Key>
    Delete(&'a [u8]),
}

/// Transaction log representation with its associated metadata
///
/// the metadata is the timestamp and parent reference used in `replay` and `replayt`
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Node<'a> {
    #[serde(borrow)]
    transaction: Transaction<'a>,
    /// current epoch time
    timestamp: i64,
    /// parent node in `namespace.branch_id.seq` format
    parent: Option<Vec<u8>>,
}

impl<'a> Node<'a> {
    /// creates new Node with the current unix timestamp
    fn new(transaction: Transaction<'a>) -> Self {
        trace!("create new transaction");
        Node {
            transaction,
            timestamp: Utc::now().timestamp(),
            parent: None,
        }
    }

    fn set_parent(&mut self, namespace: &str, branch: u64, parent: u64) {
        let parent_id = format!("{}.{}.{}", namespace, branch, parent);
        self.parent = Some(parent_id.into_bytes());
    }

    fn get_parent(&self) -> Option<&Vec<u8>> {
        self.parent.as_ref()
    }
}

/// Branch metadata
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Metadata {
    id: u64,
    head: u64,
    tail: u64,
}

impl Metadata {
    fn new(id: u64, head: u64, tail: u64) -> Self {
        Metadata { id, head, tail }
    }
}

/// Transaction log branch representation
///
/// it contains the associated metadata to validate and fast unwind the logs
#[derive(Serialize, Deserialize, Debug)]
struct Branch {
    metadata: Option<Metadata>,
    parent_metadata: Option<Metadata>,
}

impl Branch {
    fn new() -> Self {
        Self {
            metadata: None,
            parent_metadata: None,
        }
    }

    fn create_with_parent(parent_metadata: Metadata) -> Self {
        Self {
            metadata: None,
            parent_metadata: Some(parent_metadata),
        }
    }
}

pub struct Tree<'a, T> {
    namespace: &'a str,
    backend: T,
    // branches's hashmap format: { branch_id(seq): Branch }
    branches: Vec<Branch>,
}

impl<'a, T> Tree<'a, T> {
    pub fn new(namespace: &'a str, backend: T) -> Self {
        Tree {
            namespace,
            backend,
            branches: Vec::new(),
        }
    }
}

impl<'a, T> Tree<'a, T>
where
    T: Backend,
{
    /// adds empty branch to tree
    ///
    /// return branch id
    pub fn create_branch(&mut self) -> Result<usize> {
        let new_branch_id = self.next_branch();
        let new_branch = Branch::new();

        self.save_branch(new_branch_id, &new_branch)?;
        self.branches.push(new_branch);

        Ok(new_branch_id)
    }

    /// fork a new empty branch from parent branch and adds it to the tree
    ///
    /// return branch id
    pub fn fork(&mut self, parent_id: usize) -> Result<usize> {
        let parent_branch_metadata = match self.branches.get(parent_id) {
            Some(branch) => branch.metadata.clone(),
            None => {
                let msg = format!("parent branch {} is not found", parent_id);
                return Err(io::Error::new(io::ErrorKind::Other, msg));
            }
        };

        // create a fork form the parent only if the parent is not root
        // otherwise just create another root branch
        if let Some(parent_branch_metadata) = parent_branch_metadata {
            let new_branch_id = self.next_branch();
            let new_branch = Branch::create_with_parent(parent_branch_metadata);
            self.save_branch(new_branch_id, &new_branch)?;
            self.branches.push(new_branch);
            Ok(new_branch_id)
        } else {
            self.create_branch()
        }
    }

    /// save branch in the backend
    fn save_branch(&mut self, branch_id: usize, branch: &Branch) -> Result<()> {
        // The key of the branch in the backend
        let branch_id = format!("{}.{}", self.namespace, branch_id).into_bytes();
        let serialized_branch =
            serialize(branch).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        self.backend.push(&branch_id, &serialized_branch)?;
        Ok(())
    }

    /// returns next branch id based on self.branches last index
    fn next_branch(&self) -> usize {
        if !self.branches.is_empty() {
            self.branches.len()
        } else {
            0
        }
    }

    // TODO: move push to Branch struct
    /// stores the provided node in the backend
    ///
    /// returns node id
    pub fn push(&mut self, branch_id: usize, transaction: Transaction) -> Result<Vec<u8>> {
        let mut node = Node::new(transaction);
        let mut node_id = format!("{}.{}", self.namespace, branch_id);

        // check if branch is already created
        if let Some(ref mut branch) = self.branches.get_mut(branch_id) {
            // initialize branch if it's the first node in the branch
            if let Some(ref mut branch_metadata) = branch.metadata {
                node.set_parent(self.namespace, branch_id as u64, branch_metadata.tail);
                branch_metadata.tail += 1;
                node_id = format!("{}.{}", node_id, branch_metadata.tail);
            } else {
                trace!("branch {} is empty", branch_id);
                if let Some(ref parent_metadata) = branch.parent_metadata {
                    node.set_parent(self.namespace, parent_metadata.id, parent_metadata.tail);
                }
                branch.metadata = Some(Metadata::new(branch_id as u64, 1, 1));
                node_id = format!("{}.{}", node_id, 1);
            }

            let serialized_node =
                serialize(&node).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
            self.backend.push(node_id.as_bytes(), &serialized_node)?;
            Ok(node_id.into_bytes())
        } else {
            let msg = format!("Branch {} is not found", branch_id);
            Err(io::Error::new(io::ErrorKind::Other, msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    impl Backend for HashMap<Vec<u8>, Vec<u8>> {
        fn push(&mut self, key: &[u8], data: &[u8]) -> Result<()> {
            self.insert(key.to_vec(), data.to_vec());
            Ok(())
        }

        fn fetch(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
            let data = self.get(key);
            match data {
                Some(data) => Ok(Some(data.to_vec())),
                None => Ok(None),
            }
        }
    }

    #[test]
    fn create_branch_saves_metadata_to_backend_and_branches() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();
        // branch is saved in self.branches correctly
        assert_eq!(tlog_tree.branches.len(), 1);
        assert_eq!(branch_id, 0);
        // branch is saved in the backend with <namespace>.<branch_id> as a key
        let branch_key = format!("{}.{}", namespace, branch_id).into_bytes();
        assert!(tlog_tree.backend.contains_key(&branch_key));
    }

    #[test]
    fn fork_from_empty_branch_create_root_branch() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let forked_branch_id = tlog_tree.fork(branch_id).unwrap();
        assert!(
            tlog_tree.branches[forked_branch_id]
                .parent_metadata
                .is_none()
        );
    }

    #[test]
    #[should_panic(expected = "parent branch 10 is not found")]
    fn fork_from_non_existent_branch_return_err() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);

        tlog_tree.fork(10).unwrap();
    }

    #[test]
    fn fork_from_non_empty_branch() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction = Transaction::Set((b"hello", b"world"));
        tlog_tree.push(branch_id, transaction).unwrap();
        let transaction = Transaction::Set((b"hello", b"world"));
        tlog_tree.push(branch_id, transaction).unwrap();

        let forked_branch_id = tlog_tree.fork(branch_id).unwrap();

        let parent_metadata = &tlog_tree.branches[forked_branch_id].parent_metadata;
        assert!(parent_metadata.is_some());
        assert_eq!(parent_metadata.clone().unwrap().head, 1);
        assert_eq!(parent_metadata.clone().unwrap().tail, 2);
    }

    #[test]
    fn update_metadata_successfully_first_push() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction = Transaction::Set((b"hello", b"world"));
        tlog_tree.push(branch_id, transaction).unwrap();

        let branch_metadata = &tlog_tree.branches[branch_id].metadata;
        assert!(branch_metadata.is_some());
        assert_eq!(branch_metadata.clone().unwrap().head, 1);
        assert_eq!(branch_metadata.clone().unwrap().tail, 1);
    }

    #[test]
    fn update_metadata_successfully_multiple_pushes() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction1 = Transaction::Set((b"hello", b"world"));
        let transaction2 = Transaction::Set((b"hello2", b"world2"));
        let transaction3 = Transaction::Set((b"hello3", b"world3"));
        tlog_tree.push(branch_id, transaction1).unwrap();
        tlog_tree.push(branch_id, transaction2).unwrap();
        tlog_tree.push(branch_id, transaction3).unwrap();

        let branch_metadata = &tlog_tree.branches[branch_id].metadata;
        assert!(branch_metadata.is_some());
        assert_eq!(branch_metadata.clone().unwrap().head, 1);
        assert_eq!(branch_metadata.clone().unwrap().tail, 3);
    }

    #[test]
    fn store_data_successfully_single_push() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction = Transaction::Set((b"hello", b"world"));
        let node_key = tlog_tree.push(branch_id, transaction).unwrap();

        let deserialized_node: Node = deserialize(&tlog_tree.backend[&node_key]).unwrap();

        assert_eq!(
            deserialized_node.transaction,
            Transaction::Set((b"hello", b"world"))
        );
    }

    #[test]
    fn stored_nodes_has_valid_parent_references() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction = Transaction::Set((b"hello1", b"world1"));
        let node1_id = tlog_tree.push(branch_id, transaction).unwrap();
        let transaction = Transaction::Set((b"hello1", b"world2"));
        let node2_id = tlog_tree.push(branch_id, transaction).unwrap();

        let deserialized_node1: Node = deserialize(&tlog_tree.backend[&node1_id]).unwrap();

        let deserialized_node2: Node = deserialize(&tlog_tree.backend[&node2_id]).unwrap();

        assert!(deserialized_node1.parent.is_none());
        assert_eq!(deserialized_node2.parent, Some(b"default.0.1".to_vec()));
    }

    #[test]
    fn stored_nodes_of_fork_has_valid_parent_references() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction = Transaction::Set((b"hello1", b"world1"));
        let node1_id = tlog_tree.push(branch_id, transaction).unwrap();
        let transaction = Transaction::Set((b"hello2", b"world2"));
        let node2_id = tlog_tree.push(branch_id, transaction).unwrap();

        let forked_branch_id = tlog_tree.fork(branch_id).unwrap();
        let transaction = Transaction::Set((b"hello3", b"world3"));
        let node3_id = tlog_tree.push(forked_branch_id, transaction).unwrap();

        let deserialized_node3: Node = deserialize(&tlog_tree.backend[&node3_id]).unwrap();

        assert_eq!(node3_id, b"default.1.1".to_vec());
        assert_eq!(deserialized_node3.parent, Some(b"default.0.2".to_vec()));
    }

    #[test]
    #[ignore]
    fn store_metadata_successfully_on_max_transactions() {}
}
