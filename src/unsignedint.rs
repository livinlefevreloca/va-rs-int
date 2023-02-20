use std::{
    error::Error,
    fmt::Display,
    ops::{BitAnd, BitOr, Shl, Shr},
};

use crate::varint_serde::{
    VarIntDecode, VarIntEncode, CONTINUATION_MASK, ENCODED_CHUNK_SIZE, VALUE_MASK,
};

macro_rules! impl_var_unsigned_int_encode {
    ($int:ty) => {
        impl VarIntEncode for $int {
            fn encode(&self) -> Vec<u8> {
                let mut output = Vec::with_capacity(<u8 as VarIntEncode>::get_endcoded_byte_size());
                let mut shifted = *self;
                while shifted != 0 {
                    let val = if shifted >> ENCODED_CHUNK_SIZE | 0 as $int != 0 {
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

impl_var_unsigned_int_encode!(u8);
impl_var_unsigned_int_encode!(u16);
impl_var_unsigned_int_encode!(u32);
impl_var_unsigned_int_encode!(u64);
impl_var_unsigned_int_encode!(u128);

pub trait UnsignedInt<I>:
    Sized
    + Default
    + Copy
    + Shl<usize, Output = Self>
    + BitAnd<Self, Output = Self>
    + Shr<usize, Output = Self>
    + BitOr<Self, Output = I>
    + From<u8>
    + Display
{
}

impl UnsignedInt<u8> for u8 {}
impl UnsignedInt<u16> for u16 {}
impl UnsignedInt<u32> for u32 {}
impl UnsignedInt<u64> for u64 {}
impl UnsignedInt<u128> for u128 {}

impl<I: UnsignedInt<I>> VarIntDecode<I> for &[u8] {
    fn decode(&self) -> Result<Vec<I>, Box<dyn Error>> {
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
                let value: I = (VALUE_MASK & current).try_into()?;
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
