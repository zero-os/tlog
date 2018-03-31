// extern crate tlog;

// use std::io::Result;
// use std::collections::HashMap;

// use tlog::backend::Backend;
// use tlog::transaction::{Transaction, TransactionLog};

// /// a hashmap based backend used as a mock
// struct InMemoryBackend {
//     client: HashMap<Vec<u8>, Vec<u8>>,
// }

// impl InMemoryBackend {
//     fn new() -> Self {
//         InMemoryBackend {
//             client: HashMap::new(),
//         }
//     }
// }

// impl Backend for InMemoryBackend {
//     fn push(&mut self, key: &[u8], data: &[u8]) -> Result<()> {
//         self.client.insert(key.to_vec(), data.to_vec());
//         Ok(())
//     }

//     fn fetch(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
//         let x = self.client.get(key);
//         let y = x.unwrap().clone();
//         Ok(Some(y))
//     }
// }

// #[test]
// #[ignore]
// fn metadata_is_stored() {
//     let in_memory = InMemoryBackend::new();

//     let mut transaction1 = Transaction::set(b"testkey", b"testval");
//     let mut transaction2 = Transaction::set(b"testkey", b"testval");
//     let mut transaction_log = TransactionLog::new("test".to_owned(), in_memory);
//     transaction_log.add(&mut transaction1).unwrap();
//     transaction_log.add(&mut transaction2).unwrap();

//     transaction_log.replay();
//     assert_eq!(3, 3);
// }

// #[test]
// fn transaction_is_stored() {
//     assert_eq!(3, 3);
// }
