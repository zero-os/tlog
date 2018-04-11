use std::sync::Arc;
use std::io::{self, Result};
use tlog::Backend;
mod generated;

use self::generated::daemon as model;
use self::generated::daemon_grpc as stubs;

use grpcio::{ChannelBuilder, EnvBuilder};

pub struct Zstor {
    client: stubs::FileServiceClient,
}

impl Zstor {
    pub fn new(addr: &str) -> Self {
        let env = Arc::new(EnvBuilder::new().build());
        let channel = ChannelBuilder::new(env).connect(addr);
        let data_service = stubs::FileServiceClient::new(channel);
        Zstor {
            client: data_service,
        }
    }
}

impl Backend for Zstor {
    fn push(&mut self, key: Vec<u8>, data: Vec<u8>) -> Result<()> {
        let mut req = model::WriteRequest::new();
        req.set_key(key);
        req.set_data(data);
        self.client
            .write(&req)
            .map_err(|err| {
                error!("fetch error; {}", err);
                io::Error::new(io::ErrorKind::Other, err)
            })?;
        Ok(())
    }

    fn fetch(&self, key: Vec<u8>) -> Result<Option<Vec<u8>>> {
        let mut req = model::ReadRequest::new();
        req.set_key(key);
        let resp = self.client.read(&req);

        match resp {
            Ok(resp) => Ok(Some(resp.data)),
            Err(error) => {
                if error.to_string().contains("daemon: key is no found") {
                    Ok(None)
                } else {
                    error!("fetch error; {}", error);
                    Err(io::Error::new(io::ErrorKind::Other, error))
                }
            },
        }
    }
}
