use crate::varint_serde::{VarIntEncode, VarIntDecode, ENCODED_CHUNK_SIZE};


const CONTINUATION_MASK: u8 = 0b10000000;
const VALUE_MASK: u8 = 0b0111111;


macro_rules! impl_var_unisgned_int_encode {
    ($int:ty) => {
        impl VarIntEncode for $int {
            fn encode(&self) -> Vec<u8> {
                let mut output = Vec::with_capacity(<u8 as VarIntEncode>::get_endcoded_byte_size());
                let mut shifted = *self;
                while shifted != 0 {
                    output.push(shifted as u8);
                    shifted >>= ENCODED_CHUNK_SIZE;
                }
                output
            }
        }
    };
}

impl_var_unisgned_int_encode!(u8);
impl_var_unisgned_int_encode!(u16);
impl_var_unisgned_int_encode!(u32);
impl_var_unisgned_int_encode!(u64);
impl_var_unisgned_int_encode!(u128);



macro_rules! impl_var_unisgned_int_decode {
    ($collection:ty, $int: ty) => {
        impl VarIntDecode<$int> for $collection {
            fn decode(&self) -> Vec<$int> {
                let mut output = Vec::with_capacity(
                    <Self as VarIntDecode<$int>>::get_decoded_size(self.len())
                );
                let mut ptr = 0;
                let mut current: $int = 0; 
                loop {
                    if ptr > self.len() {
                        return output;
                    }
                    
                    while (self[ptr] & CONTINUATION_MASK) >> ENCODED_CHUNK_SIZE == 1  {
                        current |= (VALUE_MASK & self[ptr]) as $int;
                        current <<= ENCODED_CHUNK_SIZE;
                        ptr += 1
                    }
                    output.push(current);
                }
            }
        }
        
    };
}

impl_var_unisgned_int_decode!(&[u8], u8);
impl_var_unisgned_int_decode!(Vec<u8>, u8);
impl_var_unisgned_int_decode!(&[u8], u16);
impl_var_unisgned_int_decode!(Vec<u8>, u16);
impl_var_unisgned_int_decode!(&[u8], u32);
impl_var_unisgned_int_decode!(Vec<u8>, u32);
impl_var_unisgned_int_decode!(&[u8], u64);
impl_var_unisgned_int_decode!(Vec<u8>, u64);
impl_var_unisgned_int_decode!(&[u8], u128);
impl_var_unisgned_int_decode!(Vec<u8>, u128);
