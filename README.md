# Modular bitfield ordering types

This crate provide some types to use with the `modular-bitfield` crate:

For big endian layout: `u8be`, `u16be`, `u32be`, `u64be`, `u128be`  
For little endian layout: `u8le`, `u16le`, `u32le`, `u64le`, `u128le`

## Example

```rust 

#[bitfield]
struct Foo {
    a: u16be,
    b: u32le,
}

```
