use thiserror::Error;

pub(crate) const ENCODED_CHUNK_SIZE: usize = 7;
pub(crate) const CONTINUATION_MASK: u8 = 0b10000000;
pub(crate) const VALUE_MASK: u8 = 0b01111111;

#[derive(Debug, Error)]
pub enum SerdeError {
    #[error("Failed to decode num")]
    Decode(String),
}

pub trait VarIntEncode: Sized {
    fn encode(&self) -> Vec<u8>;
    fn get_endcoded_byte_size() -> usize {
        ((std::mem::size_of::<Self>() * 8) / ENCODED_CHUNK_SIZE) + 1
    }
}

pub trait VarIntDecode<I: Sized> {
    fn decode(&self) -> Result<Vec<I>, SerdeError>;
    fn get_decoded_size(byte_count: usize) -> usize {
        byte_count / std::mem::size_of::<I>()
    }
}
