# Procedural Macros for the Blockchain Tutorial

## NewType

This is a derive procedural macro for making it easy to implement the [New Type Pattern](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).

The benefit of this approach is to add implementations to a `struct` that is owned by another crate.
Rust's auto-deref and deref coercion take care of switching types automagically.

### Usage

```rust
use proc_macros::NewType;

#[derive(NewType)]
pub(crate) struct Block(SimpleBlock);
```

This appends the following `Deref` and `DerefMut` impls:

```rust
impl std::ops::Deref for Block {
    type Target = SimpleBlock;

    fn deref(&self) -> &SimpleBlock {
        &self.0
    }
}

impl std::ops::DerefMut for Block {
    fn deref_mut(&mut self) -> &mut SimpleBlock {
        &mut self.0
    }
}
```