use std::io::{BufWriter, Write};
use tlog::Transaction;

pub struct Response<'a, T: 'a>
where
    &'a T: Write,
{
    pub writer: BufWriter<&'a T>,
}

impl<'a, T> Response<'a, T>
where
    &'a T: Write,
{
    pub fn new(writer: BufWriter<&'a T>) -> Self {
        Response { writer }
    }

    pub fn send_text(&mut self, text: &str) {
        let msg = format!("+{}\r\n", text);
        self.send(msg.as_bytes());
    }

    pub fn send_transaction(&mut self, transaction: &Transaction) {
        let msg;
        match transaction {
            Transaction::Set(key, val) => {
                let key_meta = format!("${}\r\n", key.len());
                let val_meta = format!("${}\r\n", val.len());

                let mut cmd: Vec<&[u8]> = vec![];

                cmd.push(b"*3\r\n$3\r\nSET\r\n");
                cmd.push(key_meta.as_bytes());
                cmd.push(&key);
                cmd.push(b"\r\n");
                cmd.push(val_meta.as_bytes());
                cmd.push(&val);
                cmd.push(b"\r\n");
                msg = cmd.concat();
            }
            Transaction::Delete(key) => {
                let key_meta = format!("${}\r\n", key.len());

                let mut cmd: Vec<&[u8]> = vec![];

                cmd.push(b"*2\r\n$3\r\nDEL\r\n");
                cmd.push(key_meta.as_bytes());
                cmd.push(&key);
                cmd.push(b"\r\n");

                msg = cmd.concat();
            }
        };
        self.send(&msg);
    }

    fn send(&mut self, payload: &[u8]) {
        self.writer.write_all(payload).unwrap_or_else(|err| error!("error send; {}", err));
        self.writer.flush().unwrap_or_else(|err| error!("error send; {}", err));
    }
}
