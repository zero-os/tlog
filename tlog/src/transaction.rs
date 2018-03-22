// Transaction
//    - payload
//    - parent
//    - timestamp

// Branch
//    - Metadata { tail, head }
//    - ParentMetadata: Option<Metadata>

// Tree
// fn push: (i/p) branch_id (default), transaction -> transaction_id
// fn fork: (i/p) parent_branch -> new_branch_id
// fn scan: (i/p) branch_id (default), start_transaction_id, stop_transaction_id, stop_timestamp -> the whole tree seq t6 -> t5 -> t2 -> t1
// fn replay: start_transaction_id, stop_transaction_id -> (iterator)
// fn replayt: start_timestamp, stop_timestamp -> (iterator)

use chrono::prelude::*;
use std::io::{self, Result};
use bincode::{deserialize, serialize};
use backend::Backend;

const MAX_TRANSACTIONS_TO_FLUSH: u64 = 10;

#[derive(Serialize, Deserialize, Debug)]
enum Payload<'a> {
    Set((&'a [u8], &'a [u8])),
    Delete(&'a [u8]),
}

/// Transaction
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction<'a> {
    #[serde(borrow)]
    payload: Payload<'a>,
    /// current epoch time
    timestamp: i64,
    /// next is in `namespace.seq` format
    next: Option<String>,
    /// prev is in `namespace.seq` format
    prev: Option<String>,
}

impl<'a> Transaction<'a> {
    /// Creates a `Set` transaction
    pub fn set(key: &'a [u8], data: &'a [u8]) -> Self {
        Transaction {
            payload: Payload::Set((key, data)),
            timestamp: Utc::now().timestamp(),
            next: None,
            prev: None,
        }
    }

    /// Creates a `Delete` transaction
    pub fn delete(&mut self, key: &'a [u8]) -> Self {
        Transaction {
            payload: Payload::Delete(key),
            timestamp: Utc::now().timestamp(),
            next: None,
            prev: None,
        }
    }
}

/// `TransactionLog` metadata
///
/// contains the metadata needed to initialize and scan the chain
#[derive(Serialize, Deserialize)]
struct Metadata {
    length: u64,
    /// head is in `namespace.1`
    head: Option<String>,
    /// tail is in `namespace.<lenght>`
    tail: Option<String>,
}

/// `TransactionLog` contains the logic
pub struct TransactionLog<T>
where
    T: Backend,
{
    backend: T,
    namespace: String,
    metadata: Metadata,
    /// used as a pointer to the current node in iterator impl
    current_seq: u64,
}

impl<T> TransactionLog<T>
where
    T: Backend,
{
    pub fn new(namespace: String, backend: T) -> Self {
        let metadata = Metadata {
            length: 0,
            head: None,
            tail: None,
        };

        TransactionLog {
            backend,
            namespace,
            metadata,
            current_seq: 0,
        }
    }
}

impl<T: Backend> TransactionLog<T> {
    pub fn add(&mut self, log: &mut Transaction) -> Result<()> {
        // TODO: implement the flushing logic as a future
        // TODO: add a time interval for the flush
        // store metadata if the chain is empty or it reached MAX_TRANSACTIONS_TO_FLUSH
        if self.metadata.length % MAX_TRANSACTIONS_TO_FLUSH == 0 {
            debug!("{}: metadata is stored", self.namespace);
            let key = format!("{}.metadata", self.namespace);
            let serialized_tlog =
                serialize(&self.metadata).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
            self.backend.push(key.as_bytes(), &serialized_tlog)?;
        }

        let key = format!("{}.{}", self.namespace, self.metadata.length + 1);

        // update head if it's the first transaction
        if self.metadata.head.is_none() {
            self.metadata.head = Some(key.clone());
        } else {
            log.prev = self.metadata.tail.clone();
        }

        // store the transaction
        let serialized_log =
            serialize(&log).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.backend.push(key.as_bytes(), &serialized_log)?;

        match self.metadata.tail {
            Some(ref tail) => {
                // update the head of prev transaction log
                let data = self.backend
                    .fetch(tail.as_bytes())?
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "data is not found"))?;
                let mut decoded_head: Transaction = deserialize(&data[..])
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                decoded_head.next = Some(key.clone());

                let serialized_log = serialize(&decoded_head)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                self.backend.push(key.as_bytes(), &serialized_log)?;
            }
            None => {}
        };

        // update the metadata
        self.metadata.length += 1;
        self.metadata.tail = Some(key.clone());

        Ok(())
    }

    // TODO: return stream using futures
    /// replay all the transaction logs in an ascending order
    pub fn replay(self) -> Result<Option<Vec<Vec<u8>>>> {
        match self.metadata.head {
            Some(head) => {
                let data = self.backend
                    .fetch(head.as_bytes())?
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "data is not found"))?;

                let decoded_head: Transaction = deserialize(&data[..])
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

                // return result;
                Err(io::Error::new(io::ErrorKind::Other, "malformed request"))
            }
            None => Ok(None),
        }
    }
}

impl Iterator for TransactionLog {
    type Item = Vec<u8>;

    fn next() -> Option<Self::Item> {
        match self.metadata.head {
            Some(head) => {
                let data = self.backend
                    .fetch(head.as_bytes())?
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "data is not found"))?;

                let decoded_head: Transaction = deserialize(&data[..])
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

                // return result;
                Err(io::Error::new(io::ErrorKind::Other, "malformed request"))
            }
            None => None,
        }
    }
}
