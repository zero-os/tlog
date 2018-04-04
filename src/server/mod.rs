use std::io::Result;
use std::net::TcpListener;

use backends::zstor::Zstor;
use tlog::{Transaction, Tree};

mod connection;
use self::connection::Connection;

/// Supported commands
#[derive(Debug)]
pub enum Command {
    PING,
    Set(Vec<u8>, Vec<u8>),
    Delete(Vec<u8>),
    Replay(Vec<u8>),
    BranchNew,
    BranchFork(usize),
    Branch(usize),
    NOTSUPPORTED,
}

/// TCP server that is responsible for managing the connections
pub struct Server<'a> {
    listener: TcpListener,
    tree: Tree<'a, Zstor>,
    /// active branch on the tree
    branch: usize,
}

impl<'a> Server<'a> {
    /// Creates new server with specified addr
    ///
    /// addr should be full address, ie: `localhost:10809`
    pub fn new(addr: &str, zstor_addr: &str, namespace: &'a str, branch: usize) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        let storage = Zstor::new(zstor_addr);
        let tree = Tree::new(namespace, storage);

        // if branch != 0 && !tree.branch_exists(branch) {
        //     panic!("branch {} does not exist", branch);
        // } else {
        //     tree.create_branch().unwrap();
        // }
        // info!("branches: loading branch {}", branch);

        Server {
            listener,
            tree,
            branch,
        }
    }

    /// Listens on address in self.listener and
    ///
    /// it uses single thread approach to manage all the connections
    pub fn serve(&mut self) -> Result<()> {
        info!("Listen on: {}", self.listener.local_addr().unwrap());
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    info!("accepted connection; addr={}", stream.peer_addr()?);
                    let mut conn = Connection::new(&stream);
                    for cmd in conn.reader.into_iter() {
                        match cmd {
                            Command::PING => {
                                conn.writer.send_text("PONG")?;
                            }
                            Command::Set(k, v) => {
                                let trans = Transaction::Set(k, v);
                                // self.tree.push(1, trans);
                            }
                            _ => {
                                println!("{:?}", cmd);
                            }
                        }
                    }
                    info!("closing connection; addr={}", stream.peer_addr()?);
                }
                Err(e) => {
                    error!("accept error = {:?}", e);
                }
            }
        }
        Ok(())
    }
}
