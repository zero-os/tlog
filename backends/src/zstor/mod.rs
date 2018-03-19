use std::sync::Arc;
use std::io::{self, Result};
use tlog::backend::Backend;
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
    fn push(&mut self, key: &[u8], data: &[u8]) -> Result<()> {
        let mut req = model::WriteRequest::new();
        req.set_key(key.to_vec());
        req.set_data(data.to_vec());
        self.client
            .write(&req)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        Ok(())
    }
}
