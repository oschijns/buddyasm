use bitvec::{order::BitOrder, vec::BitVec};
use bytes::{BufMut, Bytes, BytesMut};

/// Implement methods to convert bitvec into bytes according to a specific endianness encoding
macro_rules! impl_to_bytes {
    ($func:ident => $inner:ident($int:ty)) => {
        pub fn $func<O>(bits: &BitVec<$int, O>) -> Bytes
        where
            O: BitOrder,
        {
            // Compose a sequence of bytes.
            let mut bytes = BytesMut::with_capacity(bits.len() / 8);

            // Iterate over each word in the bitvec.
            let _ = bits.as_raw_slice().iter().map(|word| {
                // Encode the word into the byte sequence while using the expected endianness.
                bytes.$inner(*word);
            });

            // Sequence of bytes is done.
            bytes.freeze()
        }
    };
}

impl_to_bytes! { bits_to_u16_le => put_u16_le(u16) }
impl_to_bytes! { bits_to_u16_be => put_u16   (u16) }
impl_to_bytes! { bits_to_u32_le => put_u32_le(u32) }
impl_to_bytes! { bits_to_u32_be => put_u32   (u32) }
impl_to_bytes! { bits_to_u64_le => put_u64_le(u64) }
impl_to_bytes! { bits_to_u64_be => put_u64   (u64) }
