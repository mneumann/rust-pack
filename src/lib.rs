use std::mem;
use std::convert::From;

#[derive(Debug, Eq, PartialEq)]
pub struct U32b([u8; 4]);

impl<'a> From<&'a [u8]> for U32b {
    fn from(a: &[u8]) -> U32b {
        assert!(a.len() == 4);
        U32b([a[0], a[1], a[2], a[3]])
    }
}

#[inline(always)]
pub fn pack_native_u32(n: u32) -> U32b {
    U32b(unsafe { mem::transmute(n) })
}

#[inline(always)]
pub fn unpack_native_u32(bytes: U32b) -> u32 {
    match bytes {
        U32b(b) => unsafe { mem::transmute(b) }
    }
}

#[cfg(target_endian = "little")]
pub fn pack_le_u32(n: u32) -> U32b {
    pack_native_u32(n)
}

#[cfg(target_endian = "little")]
pub fn unpack_le_u32(bytes: U32b) -> u32 {
    unpack_native_u32(bytes)
}

#[cfg(target_endian = "big")]
pub fn pack_le_u32(n: u32) -> U32b {
    pack_native_u32(n.swap_bytes())
}

#[cfg(target_endian = "big")]
fn unpack_le_u32(bytes: U32b) -> u32 {
    unpack_native_u32(bytes).swap_bytes()
}

#[test]
fn test_pack() {
    assert_eq!(U32b([123, 0, 0, 0]), pack_le_u32(123));
}

#[test]
fn test_unpack() {
    assert_eq!(123, unpack_le_u32(U32b([123, 0, 0, 0])));

    let b = [123, 0, 0, 0, 0u8];
    assert_eq!(123, unpack_le_u32(U32b::from(&b[0..4])));
}
