# TLog

Tlog is a generic transaction log framework over any key, value backend storage

# Implementation

Transaction logs are stored as a tree-like structure to support forks. Every node will only have a reference to its parent node, so that at `playback` the path of the whole transaction log can be calculated beforehand. Every branch will have a reference to the metadata of its parent tree at the time of the fork, which contains the `head` and `tail` nodes, and that enables branch rebasing.

## Use Case:

If `source1` starts to push two transaction logs the data would be stored as below and with every additions the user will get the `transaction_id` as a return

```
                        source1 (default)
                        t1
                        |
                        t2
```

and when another source, let's call it `source2`, creates a fork of `source1`'s tree the `branch_id` and started added transaction logs, while `source1` also adds more transaction logs, a new branch will be added for `source2` as below

```
                    source1(default)              source2
                        t1
                        |
                        t2__________________________
                        |                           |
                        t3                          t5
                        |                           |
                        t4                          t6
```

Tlog supports transaction logs playback based on both timestamp and sequence.

Firstly we'll look at the sequence playback, the user should provide `branch_id`, start `transaction_id`, and the optional end `transaction_id`. Then the whole tree would be calculated first, in the case of `source2` the tree would look like this `t6 -> t5 -> t2 -> t1`, from which the transaction logs can be played back to the user.

Secondly the timestamp playback, this is tricky because transactions' timestamp are stored as a field in the transaction not within the key. To solve this, we will start scanning the branch from its tail until the required timestamp is reached.

# Storage

Tlog supports key, value storages out of the box, you only have to implment the `Backend` trait to integrate with it.

```rust
extern crate tlog;

use tlog::backend::Backend;

struct KV;

impl Backend for KV {
    //...
}
```

# Supported Commands

* `SET`
* `DELETE`

# TODO:

* Branch rebasing
