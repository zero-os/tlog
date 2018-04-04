use super::Command;

use std::io::prelude::*;
use atoi::atoi;
use hex;
use std::io::{self, BufReader, Result};

pub struct Request<'a, T: 'a>
where
    &'a T: Read,
{
    pub reader: BufReader<&'a T>,
}

impl<'a, T> Request<'a, T>
where
    &'a T: Read,
{
    pub fn new(reader: BufReader<&'a T>) -> Self {
        Request { reader }
    }

    pub fn decode(&mut self) -> Result<Option<Command>> {
        if let Some(line) = self.read_line()? {
            let mut line_iter = line.iter();

            if line_iter.next() != Some(&b'*') {
                return Err(io::Error::new(io::ErrorKind::Other, "not an array"));
            }

            let argc = line_iter
                .next()
                .and_then(|arg| atoi::<u8>(&[*arg]))
                .ok_or(io::Error::new(io::ErrorKind::Other, "Malformed request"))?;

            let mut args: Vec<Vec<u8>> = Vec::new();

            for _i in 0..argc {
                self.read_line()?
                    .and_then(|line| {
                        if line.starts_with(&[b'$']) {
                            Some(line)
                        } else {
                            None
                        }
                    })
                    .and_then(|line| atoi::<usize>(line.get(1..)?))
                    .and_then(|size: usize| {
                        let mut line = self.read_exact(size + 2).ok()??;
                        line.truncate(size);
                        args.push(line);
                        Some(0)
                    })
                    .ok_or(io::Error::new(io::ErrorKind::Other, "Malformed request"))?;
            }

            match &args[0][..] {
                b"PING" => Ok(Some(Command::PING)),
                b"SET" => {
                    if args.len() < 2 {
                        return Err(io::Error::new(io::ErrorKind::Other, "Malformed request"));
                    }
                    Ok(Some(Command::Set(args.swap_remove(1), args.swap_remove(2))))
                }
                b"DELETE" => {
                    if args.len() < 1 {
                        return Err(io::Error::new(io::ErrorKind::Other, "Malformed request"));
                    }

                    let hash = match hex::decode(&args[1]) {
                        Ok(hash) => hash,
                        Err(_) => {
                            return Err(io::Error::new(io::ErrorKind::Other, "Malformed request"));
                        }
                    };
                    Ok(Some(Command::Delete(hash)))
                }
                b"REPLAY" => {
                    if args.len() < 1 {
                        return Err(io::Error::new(io::ErrorKind::Other, "Malformed request"));
                    }

                    let hash = match hex::decode(&args[1]) {
                        Ok(hash) => hash,
                        Err(_) => {
                            return Err(io::Error::new(io::ErrorKind::Other, "Malformed request"));
                        }
                    };
                    Ok(Some(Command::Replay(hash)))
                }
                _ => Ok(Some(Command::NOTSUPPORTED)),
            }
        } else {
            Ok(None)
        }
    }

    fn read_line(&mut self) -> Result<Option<Vec<u8>>> {
        let mut line = vec![];
        self.reader.read_until(b'\n', &mut line)?;
        line.pop();
        line.pop();
        if !line.is_empty() {
            Ok(Some(line))
        } else {
            Ok(None)
        }
    }

    fn read_exact(&mut self, size: usize) -> Result<Option<Vec<u8>>> {
        let mut line = vec![0u8; size];
        self.reader.read_exact(&mut line)?;

        if !line.is_empty() {
            Ok(Some(line))
        } else {
            Ok(None)
        }
    }
}

pub struct RequestIntoIterator<'a, 'b, T: 'a>
where
    &'a T: Read + Write,
    'a: 'b,
{
    request: &'b mut Request<'a, T>,
}

impl<'a, 'b, T> IntoIterator for &'b mut Request<'a, T>
where
    &'a T: Read + Write,
    'a: 'b,
{
    type Item = Command;
    type IntoIter = RequestIntoIterator<'a, 'b, T>;

    fn into_iter(self) -> Self::IntoIter {
        RequestIntoIterator { request: self }
    }
}

impl<'a, 'b, T> Iterator for RequestIntoIterator<'a, 'b, T>
where
    &'a T: Read + Write,
    'a: 'b,
{
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        match self.request.decode() {
            Ok(cmd) => {
                return cmd;
            }
            Err(err) => {
                error!("decoding error: {}", err);
            }
        }
        None
    }
}
