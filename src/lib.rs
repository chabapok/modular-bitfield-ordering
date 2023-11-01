#![no_std]
#![allow(non_camel_case_types)]

use modular_bitfield::Specifier;
use modular_bitfield::error::{InvalidBitPattern, OutOfBounds};

pub struct u8be;
pub struct u16be;
pub struct u32be;
pub struct u64be;
pub struct u128be;

pub struct u8le;
pub struct u16le;
pub struct u32le;
pub struct u64le;
pub struct u128le;


macro_rules! impl_ordering_specifier {
    ( $( ($typ:ty, $prim:ty: $bits:literal, $to_func:ident, $from_func:ident) ),* $(,)? ) => {
        $(
            impl Specifier for $typ {
                const BITS: usize = $bits;
                type Bytes = $prim;
                type InOut = $prim;

                #[inline]
                fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, OutOfBounds> {
                    Ok(input.$to_func())
                }

                #[inline]
                fn from_bytes(bytes: Self::Bytes) -> Result<Self::InOut, InvalidBitPattern<Self::Bytes>> {
                    Ok(<$prim>::$from_func(bytes))
                }
            }
        )*
    };
}

impl_ordering_specifier!(
    (u8be, u8: 8, to_be, from_be),
    (u16be, u16: 16, to_be, from_be),
    (u32be, u32: 32, to_be, from_be),
    (u64be, u64: 64, to_be, from_be),
    (u128be, u128: 128, to_be, from_be),

    (u8le, u8: 8, to_le, from_le),
    (u16le, u16: 16, to_le, from_le),
    (u32le, u32: 32, to_le, from_le),
    (u64le, u64: 64, to_le, from_le),
    (u128le, u128: 128, to_le, from_le),
);


#[cfg(test)]
mod tests {
    use super::*;
    use modular_bitfield::{bitfield};

    #[bitfield]
    struct Foo {
        x8: u8be,
        x16: u16be,
        x32: u32be,
        x64: u64be,
        x128: u128be,

        c8: u8le,
        c16: u16le,
        c32: u32le,
        c64: u64le,
        c128: u128le,

    }

    #[test]
    fn it_works() {
        let _ = Foo::new().into_bytes();
        let arr = [
            42,
            0x16u8, 0xd2,
            0x37, 0x38, 0x39, 0x3a,
            0x41,0x42,0x43,0x44, 0x45,0x46,0x47,0x48,
            0x50,0x51,0x52,0x53, 0x54,0x55,0x56,0x57, 0x58,0x59,0x5a,0x5b, 0x5c,0x5d,0x5e,0x5f,

            42,
            0x16u8, 0xd2,
            0x37, 0x38, 0x39, 0x3a,
            0x41,0x42,0x43,0x44, 0x45,0x46,0x47,0x48,
            0x50,0x51,0x52,0x53, 0x54,0x55,0x56,0x57, 0x58,0x59,0x5a,0x5b, 0x5c,0x5d,0x5e,0x5f,
        ];
        let foo = Foo::from_bytes(arr);

        assert_eq!(foo.x8(), 42);
        assert_eq!(foo.x16(), 0x16d2);
        assert_eq!(foo.x32(), 0x3738393a);
        assert_eq!(foo.x64(), 0x4142434445464748);
        assert_eq!(foo.x128(), 0x505152535455565758595a5b5c5d5e5f);

        assert_eq!(foo.c8(), 42);
        assert_eq!(foo.c16(), 0xd216);
        assert_eq!(foo.c32(), 0x3a393837);
        assert_eq!(foo.c64(), 0x4847464544434241);
        assert_eq!(foo.c128(), 0x5f5e5d5c5b5a59585756555453525150);
    }
}
