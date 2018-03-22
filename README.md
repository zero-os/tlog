# Tlog

tlog is a super fast & efficient transaction log server which implements redis protocol

# Implementation

> The below description is only for MVP and is still a work in progress

A rudimental and very simplified redis protocol is supported, allowing only few commands. [see below](#supported-redis-commands).

The transactions are stored in the backend as a `doubly linked list` which is serialized using a binary zero-fluff encoding scheme. The metadata of the `namespace` is also stored. This all to prevent data loss in case of a server shutdown.

We utilize Rust Trait system to implement the backend as a pluggable component but [0-stor](https://github.com/zero-os/0-stor) is the focus of this server. The server connects to a `0-stor` daemon using `grpc` which is achieved by [grpc-rs](https://github.com/pingcap/grpc-rs) crate.

# Project layout

We choose to separate the server, transaction log logic, and backend into separate crates which provides us with flexibility along with the ability to publish the logic as a separate crate to be used by the community.

The `tlog-server` crate, found at the root, is primarily the server that glues the sub crates together and implements the redis protocol.

The crates included as part of tlog-server are:

* tlog: The logic and structure in which logs are stored
* backends: backend representations to be used by `tlog` crate

# Supported redis commands

tlog will support the below commands:

* `SET key value` returns the sequence ID and the timestamp
* `DEL key` returns the sequence ID and the timestamp
* `REPLAYS <key> <start_seq> [stop_seq]` when no stop_seq is defined it is assumed that we want to playback in ascending order until the end
* `REPLAYT <key> <start_timestamp> [stop_timestamp]` when no stop_timestamp is defined it is assumed that we want to playback in ascending order until the end
* `SELECT <namespace>`

# Namespace

namespace provides an ergonimc approach to store transactions logs of multiple objects using the same server

# Challenges

### **As the transaction logs are stored as a linkedlist in the backend, when adding a new node to the chain we fetch the previous node to set the `next` field which also includes serializing and deserializing, and hence the overhead to adding nodes is big**

To solve this problem the `next` field is set from the start since transaction logs are stored with a sequential key.
