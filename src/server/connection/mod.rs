mod request;
mod response;

use self::request::Request;
use self::response::Response;
use super::Command;

use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Write};

/// Manages all operations throughout the lifetime of the connection
/// from negotiation to transmission
///
/// Both reader and writer fields can be of any stream
/// ie: `TcpStream`, `UnixStream`
pub struct Connection<'a, T: 'a>
where
    &'a T: Read + Write,
{
    pub reader: Request<'a, T>,
    pub writer: Response<'a, T>,
}

impl<'a, T> Connection<'a, T>
where
    &'a T: Read + Write,
{
    /// Creates a new connection with reader and writer
    pub fn new(stream: &'a T) -> Self {
        Connection {
            reader: Request::new(BufReader::new(stream)),
            writer: Response::new(BufWriter::new(stream)),
        }
    }
}
