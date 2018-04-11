use redis::{self, Commands};
use std::io::{self, Result};
use tlog::Backend;


pub struct Redis {
    client: redis::Connection,
}

impl Redis {
    pub fn new(addr: &str) -> Result<Self> {
        let addr = format!("redis://{}", addr);
        let client = redis::Client::open(&addr[..]).map_err(|err| {
                error!("create connection error; {}", err);
                io::Error::new(io::ErrorKind::Other, err)
            })?;

        let con = client.get_connection().map_err(|err| {
                error!("create connection error; {}", err);
                io::Error::new(io::ErrorKind::Other, err)
            })?;

        Ok(Self {
            client: con,
        })
    }
}

impl Backend for Redis {
    fn push(&mut self, key: Vec<u8>, data: Vec<u8>) -> Result<()> {
        let _resp: String = self.client.set(key, data).map_err(|err| {
                error!("push error; {}", err);
                io::Error::new(io::ErrorKind::Other, err)
            })?;
        Ok(())
    }

    fn fetch(&self, key: Vec<u8>) -> Result<Option<Vec<u8>>> {
        let resp = self.client.get(key).map_err(|err| {
                error!("fetch error; {}", err);
                io::Error::new(io::ErrorKind::Other, err)
            })?;
        Ok(Some(resp))
    }
}
