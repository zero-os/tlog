# Tlog
tlog is a super fast & efficient transaction log server which utilizes redis protocol

# Implementation
A rudimental and very simplified redis protocol is supported, allowing only few commands. See below.

The data will have `timestamp, data` fields and sent to the backend to be stored.

# Supported redis command
tlog will support the below commands:
* `SET value` where `value` is the log data to be stored
* `DEL key` where the key is the `sha256` hash of the value stored

# Namespace
namespace provides an ergonimc approach to store transactions logs of multiple objects using the same server
