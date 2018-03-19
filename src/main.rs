extern crate tlog;

use std::collections::HashMap;
use std::io::{self, Result};

use tlog::backend::Backend;
use tlog::transaction::{Transaction, TransactionLog};

struct TestBackend {
    chain: HashMap<Vec<u8>, Vec<u8>>,
}

impl TestBackend {}

impl Backend for TestBackend {
    fn push(&mut self, key: &[u8], data: &[u8]) -> Result<()> {
        self.chain.insert(key.to_vec(), data.to_vec());
        Ok(())
    }
}

fn main() {
    // create transaction
    let trans1 = Transaction::set(b"hello", b"world");
    let trans2 = Transaction::set(b"khaled", b"karam");

    let backend = TestBackend {
        chain: HashMap::new(),
    };
    let mut transaction_log = TransactionLog::new("test".to_owned(), backend);

    transaction_log.add(&trans1).unwrap();
    transaction_log.add(&trans2).unwrap();
}
