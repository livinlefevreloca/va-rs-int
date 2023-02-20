use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, Shl, Shr},
};

use crate::varint_serde::{
    SerdeError, VarIntDecode, VarIntEncode, CONTINUATION_MASK, ENCODED_CHUNK_SIZE, VALUE_MASK,
};

trait ToUnsigned {
    type Output;
    fn cast(&self) -> Self::Output;
}

macro_rules! impl_to_unsigned {
    ($from:ty, $to:ty) => {
        impl ToUnsigned for $from {
            type Output = $to;
            fn cast(&self) -> $to {
                *self as Self::Output
            }
        }
    };
}

impl_to_unsigned!(u8, u8);
impl_to_unsigned!(u16, u16);
impl_to_unsigned!(u32, u32);
impl_to_unsigned!(u64, u64);
impl_to_unsigned!(u128, u128);
impl_to_unsigned!(i8, u8);
impl_to_unsigned!(i16, u16);
impl_to_unsigned!(i32, u32);
impl_to_unsigned!(i64, u64);
impl_to_unsigned!(i128, u128);

macro_rules! impl_varint_encode {
    ($int:ty) => {
        impl VarIntEncode for $int {
            fn encode(&self) -> Vec<u8> {
                let mut output = Vec::with_capacity(<u8 as VarIntEncode>::get_endcoded_byte_size());
                let mut shifted = self.cast();
                if shifted == 0 {
                    output.push(shifted as u8)
                }

                while shifted != 0 {
                    let val = if shifted >> ENCODED_CHUNK_SIZE != 0 {
                        shifted as u8 | CONTINUATION_MASK
                    } else {
                        shifted as u8
                    };
                    output.push(val);
                    shifted >>= ENCODED_CHUNK_SIZE;
                }
                output
            }
        }
    };
}

impl_varint_encode!(u8);
impl_varint_encode!(u16);
impl_varint_encode!(u32);
impl_varint_encode!(u64);
impl_varint_encode!(u128);
impl_varint_encode!(i8);
impl_varint_encode!(i16);
impl_varint_encode!(i32);
impl_varint_encode!(i64);
impl_varint_encode!(i128);

pub trait Encodable<I>:
    Sized
    + Default
    + Copy
    + Shl<usize, Output = Self>
    + BitAnd<Self, Output = Self>
    + Shr<usize, Output = Self>
    + BitOr<Self, Output = I>
    + TryFrom<u8>
    + Display
    + std::fmt::Binary
{
}
macro_rules! impl_encodable {
    ($int:ty) => {
        impl Encodable<$int> for $int {}
    };
}

impl_encodable!(u8);
impl_encodable!(u16);
impl_encodable!(u32);
impl_encodable!(u64);
impl_encodable!(u128);
impl_encodable!(i8);
impl_encodable!(i16);
impl_encodable!(i32);
impl_encodable!(i64);
impl_encodable!(i128);

impl<I: Encodable<I>> VarIntDecode<I> for &[u8] {
    fn decode(&self) -> Result<Vec<I>, SerdeError> {
        let mut output =
            Vec::with_capacity(<Self as VarIntDecode<I>>::get_decoded_size(self.len()));
        let mut ptr = 0;
        let mut accum: I = <I as Default>::default();
        loop {
            if ptr == self.len() {
                return Ok(output);
            }
            let mut shifts = 0;
            loop {
                let current = self[ptr];
                let value: I = (VALUE_MASK & current)
                    .try_into()
                    .map_err(|_| SerdeError::Decode("Failed to convert".to_string()))?;
                let shifted_val = value << (shifts * ENCODED_CHUNK_SIZE);

                accum = accum | shifted_val;
                ptr += 1;

                if ptr == self.len() || (current & CONTINUATION_MASK) >> ENCODED_CHUNK_SIZE == 0 {
                    break;
                }

                shifts += 1;
            }
            output.push(accum);
            accum = <I as Default>::default();
        }
    }
}
