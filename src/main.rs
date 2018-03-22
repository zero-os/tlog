extern crate backends;
extern crate tlog;

use backends::zstor;

use tlog::transaction::{Transaction, TransactionLog};

fn main() {
    // create transaction
    let trans1 = Transaction::set(b"hello", b"world");
    let trans2 = Transaction::set(b"khaled", b"karam");

    let backend = zstor::Zstor::new("127.0.0.1:8080");
    let mut transaction_log = TransactionLog::new("test".to_owned(), backend);

    transaction_log.add(&trans1).unwrap();
    transaction_log.add(&trans2).unwrap();
}

