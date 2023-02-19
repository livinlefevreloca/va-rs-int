pub(crate) const ENCODED_CHUNK_SIZE: usize = 7;

pub trait VarIntEncode: Sized {
    fn encode(&self) -> Vec<u8>;
    fn get_endcoded_byte_size() -> usize {
        ((std::mem::size_of::<Self>() * 8) / ENCODED_CHUNK_SIZE) + 1
    }
}

pub trait VarIntDecode<I: Sized> {
    fn decode(&self) -> Vec<I>;
    fn get_decoded_size(byte_count: usize) -> usize {
        byte_count / std::mem::size_of::<I>()
    }
}

