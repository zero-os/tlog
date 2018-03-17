use chrono::prelude::*;

enum Payload<'a> {
    Set((&'a [u8], &'a [u8])),
    Delete(&'a [u8]),
}

pub struct Transaction<'a> {
    payload: Option<Payload<'a>>,
    timestamp: DateTime<Utc>,
}

impl<'a> Transaction<'a> {
    pub fn new() -> Self {
        Transaction {
            payload: None,
            timestamp: Utc::now(),
        }
    }

    pub fn add(&mut self, key: &'a [u8], data: &'a [u8]) {
        self.payload = Some(Payload::Set((key, data)));
    }

    pub fn remove(&mut self, data: &'a [u8]) {
        self.payload = Some(Payload::Delete(data));
    }
}
