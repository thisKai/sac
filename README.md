# Sac
A rust macro for constructing collections, including map literals with colons

## Basic usage

```rust
#[macro_use]
extern crate sac;

use std::collections::HashMap;

fn main() {
    let vec: Vec<_> = sac![1, 2, 3, 4];
    let hash_map: HashMap<_, _> = sac! {
        0: "value0",
        1: "value1",
    };
}
```
