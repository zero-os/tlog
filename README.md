# Tlog

tlog is a fast & efficient redis protocol based transaction log server to store operations taken place on any storage solution


# Implementation

A very simplified redis protocol is supported, allowing only few commands. [see below](#supported-redis-commands).

The transactions are stored in the backend as a `tree` which is serialized using a binary zero-fluff encoding scheme.

We utilize Rust Trait system to implement the backend as a pluggable component but [0-stor](https://github.com/zero-os/0-stor) is the focus of this server. The server connects to a `0-stor` daemon using `grpc` which is achieved by using [grpc-rs](https://github.com/pingcap/grpc-rs) crate.

# Project layout

We choose to separate the server, transaction log logic, and backend into separate crates which provides us with flexibility to make any component pluggable, along with the ability to publish the logic as a separate crate to be used by the community.

The `tlog-server` crate, found at the root, is primarily the server that glues the sub crates together and implements the redis protocol.

The crates included as part of tlog-server are:

* tlog: the tlog crate contains the whole logic on which the `tlog-server` is built. For more info, check tlog's [README](./tlog/README.md)
* backends: backend representations to be used by `tlog` crate

# Supported redis commands

tlog will support the below commands:

* `PING`
* `SET key value`
* `DEL key`
* `REPLAY` replay the whole transaction log in acending order

# Installation

## Prerequisites

1. Tlog requires the most recent stable Rust compiler; it can be installed with
   `rustup`.

### Installing Rust compiler with `rustup`

1. Install [`rustup.rs`](https://rustup.rs/).

2. Clone the source code:

   ```sh
   git clone https://github.com/zero-os/tlog.git
   cd tlog
   ```

3. Make sure you have the right Rust compiler installed. Run

   ```sh
   rustup override set stable
   rustup update stable
   ```


## Building

```sh
cargo build --release
```

If all goes well, this should place a binary at `target/release/tlog-server`.