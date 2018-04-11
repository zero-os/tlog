# TLog

Tlog is a generic transaction log framework over any key, value backend storage

# Implementation

Transaction logs are stored as a tree-like structure to support forks. Every node will only have a reference to its parent node, so that at `replay` the path of the whole transaction log can be calculated beforehand. Every branch will have a reference to the metadata of its parent tree at the time of the fork, which contains the `head` and `tail` nodes, which enable us to support branch rebasing in the future.

If a fork from an empty branch was issued, a new root branch will be created

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
                        t3                          t1
                        |                           |
                        t4                          t2
```

Tlog supports transaction logs replay per branch path.

we'll look at the sequence replay, the user should provide `branch_id`, start . Then the whole path points would be calculated first, in the case of `source2` the tree would look like this `(source1, 2) -> (source2, 2)`, from which the transaction logs can be played back to the user.

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
* timestamp base replay: this is tricky because transactions' timestamp are stored as a field in the transaction not within the key. To solve this, we will start scanning the branch from its tail until the required timestamp is reached.
