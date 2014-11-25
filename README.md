enum_ns
==========

A Rust compiler plugin that gives enum variants within a module the old namespacing behavior.

Supplies the `#[promote_variants]` attribute that, when applied to a crate or module, will add `use Enum::{Variant1, Variant2, ..};` to the module for each `enum` definition it contains. For forward-compatibility, does not use globs.

Apply the `#[promote_variants(export)]` for the equivalent `pub use` of the above.

Usage
--------

```rust
#![feature(phase)]
#![promote_variants]

#[phase(plugin)] extern crate enum_ns;

enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
}

fn main() {
  println!("{}, {}, {}", Variant1, Variant2, Variant3);
}
