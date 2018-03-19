use chrono::prelude::*;
use std::io::{self, Result};
use bincode::{deserialize, serialize};
use tlog::backend::Backend;

const MAX_TRANSACTIONS_TO_FLUSH: u64 = 10;

#[derive(Serialize, Deserialize)]
enum Payload<'a> {
    Set((&'a [u8], &'a [u8])),
    Delete(&'a [u8]),
}

/// Transaction
#[derive(Serialize, Deserialize)]
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
        }
    }
}

impl<T: Backend> TransactionLog<T> {
    pub fn add(&mut self, log: &Transaction) -> Result<()> {
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

        // store the transaction
        let key = format!("{}.{}", self.namespace, self.metadata.length + 1);
        let serialized_log =
            serialize(log).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.backend.push(key.as_bytes(), &serialized_log)?;

        // update the metadata
        self.metadata.length += 1;
        self.metadata.tail = Some(key.clone());
        if self.metadata.head.is_none() {
            self.metadata.head = Some(key);
        }

        Ok(())
    }
}
