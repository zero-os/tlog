extern crate bincode;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;

// Tree
//     fn load: (i/p) namespace
//          -> loop over the backend to load the associated branches metadata in memory
//          -> check if all metadata are legit (if tail was the last node added), update if not. To recover from crashes
//     fn flush: (i/p) pushes all the registered branches metadata to the backend

use bincode::{deserialize, serialize};
use chrono::prelude::*;
use std::io::{self, Result};
use std::str;
use std::collections::HashMap;

pub trait Backend {
    /// pushes the data to the backend
    ///
    /// `push` should handle how the data is stored, ie: double linkedlist...etc
    fn push(&mut self, key: Vec<u8>, data: Vec<u8>) -> Result<()>;

    /// gets the data associated with key provided from the backend
    ///
    /// `fetch` should handle how the data is fetched
    ///
    /// returns Ok(None) if key is not found
    fn fetch(&self, key: Vec<u8>) -> Result<Option<Vec<u8>>>;
}

/// Payload is representation of supported commands
///
/// each command contains its associated data
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Transaction {
    /// Set <Key> <Value>
    Set(Vec<u8>, Vec<u8>),
    /// Delete <Key>
    Delete(Vec<u8>),
}

/// Transaction log representation with its associated metadata
///
/// the metadata is the timestamp and parent reference used in `replay` and `replayt`
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Node {
    transaction: Transaction,
    /// current epoch time
    timestamp: i64,
    /// parent node in `namespace.branch_id.seq` format
    parent: Option<Vec<u8>>,
}

impl Node {
    /// creates new Node with the current unix timestamp
    fn new(transaction: Transaction) -> Self {
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
}

/// Branch metadata
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct Metadata {
    id: i64,
    head: u64,
    tail: u64,
}

impl Metadata {
    fn new(id: i64, head: u64, tail: u64) -> Self {
        Metadata { id, head, tail }
    }
}

/// Transaction log branch representation
///
/// it contains the associated metadata to validate and fast unwind the logs
#[derive(Serialize, Deserialize, Debug)]
struct Branch {
    metadata: Metadata,
    parent_metadata: Option<Metadata>,
}

impl Branch {
    fn new() -> Self {
        Self {
            metadata: Metadata::new(-1, 0, 0),
            parent_metadata: None,
        }
    }

    fn create_with_parent(parent_metadata: Metadata) -> Self {
        Self {
            metadata: Metadata::new(-1, 0, 0),
            parent_metadata: Some(parent_metadata),
        }
    }
}

pub struct Tree<'a, T> {
    namespace: &'a str,
    backend: T,
    // branches's hashmap format: { branch_id(seq): Branch }
    branches: HashMap<usize, Branch>,
}

pub struct ChainIter<'a, T: 'a>
where
    T: Backend,
{
    tree: &'a Tree<'a, T>,
    branch_id: usize,
    node_index: usize,
    branch_index: usize,
}

impl<'a, T> ChainIter<'a, T>
where
    T: Backend,
{
    fn new(tree: &'a Tree<'a, T>, branch_id: usize) -> Self {
        Self {
            tree,
            branch_id,
            node_index: 1,
            branch_index: 0,
        }
    }
}

impl<'a, T> Iterator for ChainIter<'a, T>
where
    T: Backend,
{
    type Item = Result<Transaction>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chain_points = self.tree.get_chain_points(self.branch_id);
        chain_points.reverse();

        if self.branch_index < chain_points.len() {
            let (branch_key, length) = &chain_points[self.branch_index];
            if self.node_index <= *length {
                let node_key = format!("{}.{}", branch_key, self.node_index);
                trace!("Replay: fetching {}", node_key);
                let node = self.tree.backend.fetch(node_key.into_bytes()).ok()??;
                let deserialized_transaction: Transaction = deserialize(&node).ok()?;

                if self.node_index == *length {
                    self.branch_index += 1;
                    self.node_index = 0;
                }
                self.node_index += 1;
                return Some(Ok(deserialized_transaction));
            }
        }

        None
    }
}

impl<'a, T> Tree<'a, T> {
    pub fn new(namespace: &'a str, backend: T) -> Self {
        Tree {
            namespace,
            backend,
            branches: HashMap::new(),
        }
    }
}

// TODO: investigate moving Branch related methods to branch
impl<'a, T> Tree<'a, T>
where
    T: Backend,
{
    fn update_tail(&mut self, metadata: &mut Metadata) -> Result<()> {
        metadata.head = 1;
        let variant = 10;
        metadata.tail += variant;
        let node_key = format!("{}.{}.{}", self.namespace, metadata.id, metadata.tail);
        let result = self.backend.fetch(node_key.as_bytes().to_vec())?;

        if let Some(_) = result {
            self.update_tail(metadata)?;
        } else {
            let mut first = metadata.tail - variant + 1;
            let mut last = metadata.tail;

            while first <= last {
                let mut middle = (first + last) / 2;
                let node_key = format!("{}.{}.{}", self.namespace, metadata.id, middle);
                let result = self.backend.fetch(node_key.as_bytes().to_vec())?;

                if let Some(_) = result {
                    first = middle + 1;
                    metadata.tail = middle;
                } else {
                    last = middle - 1;
                }
            }
            if last == 0 {
                metadata.head = 0;
                metadata.tail = 0;
            }
        }

        Ok(())
    }

    pub fn load_branch(&mut self, branch_id: usize) -> Result<()> {
        let branch_key = format!("{}.{}", self.namespace, branch_id);
        let result = self.backend.fetch(branch_key.as_bytes().to_vec())?;

        if let Some(branch) = result {
            let mut deserialized_branch: Branch =
                deserialize(&branch).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

            deserialized_branch.metadata.id = branch_id as i64;
            self.update_tail(&mut deserialized_branch.metadata)?;

            if let Some(ref parent_metadata) = deserialized_branch.parent_metadata {
                self.load_branch(parent_metadata.id as usize)?;
            }

            self.branches.insert(branch_id, deserialized_branch);

            Ok(())
        } else {
            let msg = format!("branch {}, is not found", branch_id);
            Err(io::Error::new(io::ErrorKind::NotFound, msg))
        }
    }

    pub fn branch_exists(&self, branch_id: usize) -> bool {
        self.branches.len() > branch_id
    }

    pub fn is_empty(&self) -> bool {
        self.branches.is_empty()
    }

    /// adds empty branch to tree
    ///
    /// return branch id
    pub fn create_branch(&mut self) -> Result<usize> {
        let new_branch_id = self.next_branch();
        let new_branch = Branch::new();

        self.save_branch(new_branch_id, &new_branch)?;
        self.branches.insert(new_branch_id, new_branch);

        Ok(new_branch_id)
    }

    /// fork a new empty branch from parent branch and adds it to the tree
    ///
    /// return branch id
    pub fn fork(&mut self, parent_id: usize) -> Result<usize> {
        let parent_branch_metadata = match self.branches.get(&parent_id) {
            Some(branch) => branch.metadata.clone(),
            None => {
                let msg = format!("parent branch {} is not found", parent_id);
                return Err(io::Error::new(io::ErrorKind::Other, msg));
            }
        };

        // create a fork form the parent only if the parent is not root
        // otherwise just create another root branch
        if parent_branch_metadata.id != -1 {
            let new_branch_id = self.next_branch();
            let new_branch = Branch::create_with_parent(parent_branch_metadata);
            self.save_branch(new_branch_id, &new_branch)?;
            self.branches.insert(new_branch_id, new_branch);
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

        self.backend.push(branch_id, serialized_branch)?;
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

    /// calculate the transaction log chain recursively in a descending order
    /// which will help in replaying whole chain of the branch
    ///
    /// for example if a branch doesn't have any parent the return would be
    /// `vec![(branch_key, length)]` but if it has a parent branch
    /// the return will include the branch key and tail of the parent and so on
    /// `vec![(branch_key, length), (parent_branch_key, length)]`
    fn get_chain_points(&self, branch_id: usize) -> Vec<(String, usize)> {
        let mut chain_points = vec![];
        let branch = &self.branches[&branch_id];

        if branch.metadata.id != -1 {
            let branch_key = format!("{}.{}", self.namespace, branch_id);
            let tail_key = branch.metadata.tail as usize;
            chain_points.push((branch_key, tail_key));

            if let Some(parent_metadata) = &branch.parent_metadata {
                let mut parent_chain_points = self.get_chain_points(parent_metadata.id as usize);
                chain_points.append(&mut parent_chain_points);
            }
        }

        chain_points
    }

    // TODO: use timestamp as the key (if needed)
    // TODO: return empty result
    /// stores the provided node in the backend
    ///
    /// returns node id
    pub fn push(&mut self, branch_id: usize, transaction: Transaction) -> Result<()> {
        let mut node = Node::new(transaction);
        let mut node_id = format!("{}.{}", self.namespace, branch_id);

        // check if branch is already created
        if let Some(ref mut branch) = self.branches.get_mut(&branch_id) {
            // initialize branch if it's the first node in the branch
            if branch.metadata.id != -1 {
                node.set_parent(self.namespace, branch_id as u64, branch.metadata.tail);
                branch.metadata.tail += 1;
                node_id = format!("{}.{}", node_id, branch.metadata.tail);
            } else {
                trace!("branch {} is empty", branch_id);
                if let Some(ref parent_metadata) = branch.parent_metadata {
                    node.set_parent(
                        self.namespace,
                        parent_metadata.id as u64,
                        parent_metadata.tail,
                    );
                }
                branch.metadata = Metadata::new(branch_id as i64, 1, 1);
                node_id = format!("{}.{}", node_id, 1);
            }

            let serialized_node =
                serialize(&node).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

            self.backend.push(node_id.into_bytes(), serialized_node)?;
            Ok(())
        } else {
            let msg = format!("Branch {} is not found", branch_id);
            Err(io::Error::new(io::ErrorKind::Other, msg))
        }
    }

    pub fn replay_all(&self, branch_id: usize) -> ChainIter<T> {
        ChainIter::new(self, branch_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    impl Backend for HashMap<Vec<u8>, Vec<u8>> {
        fn push(&mut self, key: Vec<u8>, data: Vec<u8>) -> Result<()> {
            self.insert(key, data);
            Ok(())
        }

        fn fetch(&self, key: Vec<u8>) -> Result<Option<Vec<u8>>> {
            let data = self.get(&key);
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
            tlog_tree.branches[&forked_branch_id]
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

        let transaction = Transaction::Set(b"hello".to_vec(), b"world".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();
        let transaction = Transaction::Set(b"hello".to_vec(), b"world".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();

        let forked_branch_id = tlog_tree.fork(branch_id).unwrap();

        let parent_metadata = &tlog_tree.branches[&forked_branch_id].parent_metadata;
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

        let transaction = Transaction::Set(b"hello".to_vec(), b"world".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();

        let branch_metadata = &tlog_tree.branches[&branch_id].metadata;
        assert!(branch_metadata.id != -1);
        assert_eq!(branch_metadata.head, 1);
        assert_eq!(branch_metadata.tail, 1);
    }

    #[test]
    fn update_metadata_successfully_multiple_pushes() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction1 = Transaction::Set(b"hello".to_vec(), b"world".to_vec());
        let transaction2 = Transaction::Set(b"hello2".to_vec(), b"world2".to_vec());
        let transaction3 = Transaction::Set(b"hello3".to_vec(), b"world3".to_vec());
        tlog_tree.push(branch_id, transaction1).unwrap();
        tlog_tree.push(branch_id, transaction2).unwrap();
        tlog_tree.push(branch_id, transaction3).unwrap();

        let branch_metadata = &tlog_tree.branches[&branch_id].metadata;
        assert!(branch_metadata.id != -1);
        assert_eq!(branch_metadata.head, 1);
        assert_eq!(branch_metadata.tail, 3);
    }

    #[test]
    fn store_data_successfully_single_push() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction = Transaction::Set(b"hello".to_vec(), b"world".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();

        let node_key = b"default.0.1".to_vec();
        let deserialized_node: Node = deserialize(&tlog_tree.backend[&node_key]).unwrap();

        assert_eq!(
            deserialized_node.transaction,
            Transaction::Set(b"hello".to_vec(), b"world".to_vec())
        );
    }

    #[test]
    fn stored_nodes_has_valid_parent_references() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction = Transaction::Set(b"hello1".to_vec(), b"world1".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();
        let node1_id = b"default.0.1".to_vec();

        let transaction = Transaction::Set(b"hello1".to_vec(), b"world2".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();
        let node2_id = b"default.0.2".to_vec();

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

        let transaction = Transaction::Set(b"hello1".to_vec(), b"world1".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();
        let transaction = Transaction::Set(b"hello2".to_vec(), b"world2".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();

        let forked_branch_id = tlog_tree.fork(branch_id).unwrap();
        let transaction = Transaction::Set(b"hello3".to_vec(), b"world3".to_vec());
        tlog_tree.push(forked_branch_id, transaction).unwrap();
        let node3_id = b"default.1.1".to_vec();

        let deserialized_node3: Node = deserialize(&tlog_tree.backend[&node3_id]).unwrap();

        assert_eq!(node3_id, b"default.1.1".to_vec());
        assert_eq!(deserialized_node3.parent, Some(b"default.0.2".to_vec()));
    }

    #[test]
    fn replay_all() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction1 = Transaction::Set(b"hello1".to_vec(), b"world1".to_vec());
        tlog_tree.push(branch_id, transaction1.clone()).unwrap();
        let transaction2 = Transaction::Set(b"hello2".to_vec(), b"world2".to_vec());
        tlog_tree.push(branch_id, transaction2.clone()).unwrap();

        let forked_branch_id = tlog_tree.fork(branch_id).unwrap();
        let transaction3 = Transaction::Set(b"hello3".to_vec(), b"world3".to_vec());
        tlog_tree
            .push(forked_branch_id, transaction3.clone())
            .unwrap();

        let forked_branch_id = tlog_tree.fork(forked_branch_id).unwrap();
        let transaction4 = Transaction::Set(b"hello4".to_vec(), b"world4".to_vec());
        tlog_tree
            .push(forked_branch_id, transaction4.clone())
            .unwrap();

        let transaction_chain = vec![transaction1, transaction2, transaction3, transaction4];
        for (idx, x) in tlog_tree.replay_all(forked_branch_id).enumerate() {
            assert_eq!(x.unwrap(), transaction_chain[idx]);
        }
    }

    #[test]
    fn load_root_branch() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction = Transaction::Set(b"hello1".to_vec(), b"world1".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();
        let transaction = Transaction::Set(b"hello2".to_vec(), b"world2".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();
        let transaction = Transaction::Set(b"hello3".to_vec(), b"world3".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();

        tlog_tree.branches = HashMap::new();
        tlog_tree.load_branch(branch_id).unwrap();
        assert_eq!(tlog_tree.branches[&branch_id].metadata.tail, 3);
    }

    #[test]
    fn load_forked_branch() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        let transaction = Transaction::Set(b"hello1".to_vec(), b"world1".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();
        let transaction = Transaction::Set(b"hello2".to_vec(), b"world2".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();
        let transaction = Transaction::Set(b"hello3".to_vec(), b"world3".to_vec());
        tlog_tree.push(branch_id, transaction).unwrap();

        let forked_branch_id = tlog_tree.fork(branch_id).unwrap();
        let transaction = Transaction::Set(b"hello4".to_vec(), b"world4".to_vec());
        tlog_tree.push(forked_branch_id, transaction).unwrap();
        let transaction = Transaction::Set(b"hello5".to_vec(), b"world5".to_vec());
        tlog_tree.push(forked_branch_id, transaction).unwrap();

        tlog_tree.branches = HashMap::new();
        tlog_tree.load_branch(forked_branch_id).unwrap();
        assert_eq!(tlog_tree.branches[&branch_id].metadata.tail, 3);
        assert_eq!(tlog_tree.branches[&forked_branch_id].metadata.tail, 2);
    }

    #[test]
    fn load_empty_branch() {
        let namespace = "default";

        let backend: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        let mut tlog_tree = Tree::new(namespace, backend);
        let branch_id = tlog_tree.create_branch().unwrap();

        tlog_tree.branches = HashMap::new();
        tlog_tree.load_branch(branch_id).unwrap();
        assert_eq!(tlog_tree.branches[&branch_id].metadata.tail, 0);
    }

    #[test]
    #[ignore]
    fn replay_with_timestamp_limited() {}

    #[test]
    #[ignore]
    fn load_branches_metadata_in_start() {}

}
