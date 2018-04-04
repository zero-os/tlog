use std::io::{BufWriter, Result, Write};
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

    pub fn send_text(&mut self, text: &str) -> Result<()> {
        let msg = format!("+{}\r\n", text);
        self.send(msg.as_bytes())
    }

    pub fn send_transaction(&mut self, _transaction: Transaction) -> Result<()> {
        Ok(())
    }

    fn send(&mut self, payload: &[u8]) -> Result<()> {
        self.writer.write_all(payload)?;
        self.writer.flush()
    }
}
