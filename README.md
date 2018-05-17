# sac

A rust macro that will construct an instance of any collection that implements [FromIterator](https://doc.rust-lang.org/std/iter/trait.FromIterator.html).

## Cargo.toml
```toml
[dependencies]
sac = "0.2"
```

## main.rs
```rust
#[macro_use]
extern crate sac;

fn main() {
    let vec: Vec<_> = sac![1, 2, 3, 4];
    assert_eq!(vec, vec![1, 2, 3, 4]);
}
```

No type annotations are needed if the compiler can infer the types:

```rust
struct VecWrapper(Vec<i32>);

let container = VecWrapper(sac![1, 2, 3, 4]);
```

Trailing commas are also supported:

```rust
let vec: Vec<_> = sac![
    1,
    2,
    3,
    4,
];

assert_eq!(vec, vec![1, 2, 3, 4]);
```

The macro can also construct maps (e.g. [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)) with struct-like syntax:

```rust
use std::collections::HashMap;

let map_with_syntax_sugar: HashMap<_, _> = sac! {
    "foo": "bar",
    "marco": "polo",
};

let map_without_syntax_sugar: HashMap<_, _> = sac! [
    ("foo", "bar"),
    ("marco", "polo"),
];

assert_eq!(map_with_syntax_sugar, map_without_syntax_sugar);
```

Variables can be used as keys and values:

```rust
use std::collections::HashMap;

let key = "foo";
let value = "bar";

let map: HashMap<_, _> = sac! {
    key: value,
};

assert_eq!(map, sac! { "foo": "bar" });
```

To use expressions as keys, surround them with parentheses or braces:

```rust
use std::collections::HashMap;

let map: HashMap<_, _> = sac! {
    (1 + 1): "two",
    {2i32.pow(2)}: "four",
};

assert_eq!(map, sac! { 2: "two", 4: "four" });
```

License: MIT
