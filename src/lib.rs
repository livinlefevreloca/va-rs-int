use varint_serde::SerdeError;

use crate::impl_serde::Encodable;
use crate::varint_serde::VarIntDecode;

mod impl_serde;
mod varint_serde;

pub fn encode_slice<I: varint_serde::VarIntEncode>(nums: &[I]) -> Vec<u8>
where
    I: Sized,
{
    nums.iter().flat_map(|n| n.encode()).collect()
}

pub fn encode<I: varint_serde::VarIntEncode>(num: I) -> Vec<u8>
where
    I: Sized,
{
    num.encode()
}

pub fn decode<I>(bytes: &[u8]) -> Result<Vec<I>, SerdeError>
where
    I: Sized + Encodable<I>,
{
    bytes.decode()
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_encode_decode_u8() {
        let mut rng = rand::thread_rng();
        let test: u8 = rng.gen();

        let encoded = encode(test);
        let decoded = decode::<u8>(&encoded).unwrap();

        assert_eq!(test, decoded[0])
    }

    #[test]
    fn test_encode_decode_u16() {
        let mut rng = rand::thread_rng();
        let test: u16 = rng.gen();

        let encoded = encode(test);
        let decoded = decode::<u16>(&encoded).unwrap();

        assert_eq!(test, decoded[0])
    }

    #[test]
    fn test_encode_decode_u32() {
        let mut rng = rand::thread_rng();
        let test: u32 = rng.gen();

        let encoded = encode(test);
        let decoded = decode::<u32>(&encoded).unwrap();

        assert_eq!(test, decoded[0])
    }

    #[test]
    fn test_encode_decode_u64() {
        let mut rng = rand::thread_rng();
        let test: u64 = rng.gen();

        let encoded = encode(test);
        let decoded = decode::<u64>(&encoded).unwrap();

        assert_eq!(test, decoded[0])
    }

    #[test]
    fn test_encode_decode_u128() {
        let mut rng = rand::thread_rng();
        let test: u128 = rng.gen();

        let encoded = encode(test);
        let decoded = decode::<u128>(&encoded).unwrap();

        assert_eq!(test, decoded[0])
    }

    #[test]
    fn test_encode_decode_i8() {
        let mut rng = rand::thread_rng();
        let test: i8 = rng.gen();

        let encoded = encode(test);
        let decoded = decode::<i8>(&encoded).unwrap();

        assert_eq!(test, decoded[0])
    }

    #[test]
    fn test_encode_decode_i16() {
        let mut rng = rand::thread_rng();
        let test: i16 = rng.gen();

        let encoded = encode(test);
        let decoded = decode::<i16>(&encoded).unwrap();

        assert_eq!(test, decoded[0])
    }

    #[test]
    fn test_encode_decode_i32() {
        let mut rng = rand::thread_rng();
        let test: i32 = rng.gen();

        let encoded = encode(test);
        let decoded = decode::<i32>(&encoded).unwrap();

        assert_eq!(test, decoded[0])
    }

    #[test]
    fn test_encode_decode_i64() {
        let mut rng = rand::thread_rng();
        let test: i64 = rng.gen();

        let encoded = encode(test);
        let decoded = decode::<i64>(&encoded).unwrap();

        assert_eq!(test, decoded[0])
    }

    #[test]
    fn test_encode_decode_i128() {
        let mut rng = rand::thread_rng();
        let test: i128 = rng.gen();

        let encoded = encode(test);
        let decoded = decode::<i128>(&encoded).unwrap();

        assert_eq!(test, decoded[0])
    }

    #[test]
    fn test_encode_decode_u8_slice() {
        let mut rng = rand::thread_rng();
        let test: Vec<u8> = (0..10).map(|_| rng.gen()).collect();

        let encoded = encode_slice(&test);
        let decoded = decode::<u8>(&encoded).unwrap();

        assert_eq!(test, decoded)
    }

    #[test]
    fn test_encode_decode_u16_slice() {
        let mut rng = rand::thread_rng();
        let test: Vec<u16> = (0..10).map(|_| rng.gen()).collect();

        let encoded = encode_slice(&test);
        let decoded = decode::<u16>(&encoded).unwrap();

        assert_eq!(test, decoded)
    }

    #[test]
    fn test_encode_decode_u32_slice() {
        let mut rng = rand::thread_rng();
        let test: Vec<u32> = (0..10).map(|_| rng.gen()).collect();

        let encoded = encode_slice(&test);
        let decoded = decode::<u32>(&encoded).unwrap();

        assert_eq!(test, decoded)
    }

    #[test]
    fn test_encode_decode_u64_slice() {
        let mut rng = rand::thread_rng();
        let test: Vec<u64> = (0..10).map(|_| rng.gen()).collect();

        let encoded = encode_slice(&test);
        let decoded = decode::<u64>(&encoded).unwrap();

        assert_eq!(test, decoded)
    }

    #[test]
    fn test_encode_decode_u128_slice() {
        let mut rng = rand::thread_rng();
        let test: Vec<u128> = (0..10).map(|_| rng.gen()).collect();

        let encoded = encode_slice(&test);
        let decoded = decode::<u128>(&encoded).unwrap();

        assert_eq!(test, decoded)
    }

    #[test]
    fn test_encode_decode_i8_slice() {
        let mut rng = rand::thread_rng();
        let test: Vec<i8> = (0..10).map(|_| rng.gen()).collect();

        let encoded = encode_slice(&test);
        let decoded = decode::<i8>(&encoded).unwrap();

        assert_eq!(test, decoded)
    }

    #[test]
    fn test_encode_decode_i16_slice() {
        let mut rng = rand::thread_rng();
        let test: Vec<i16> = (0..10).map(|_| rng.gen()).collect();

        let encoded = encode_slice(&test);
        let decoded = decode::<i16>(&encoded).unwrap();

        assert_eq!(test, decoded)
    }

    #[test]
    fn test_encode_decode_i32_slice() {
        let mut rng = rand::thread_rng();
        let test: Vec<i32> = (0..10).map(|_| rng.gen()).collect();

        let encoded = encode_slice(&test);
        let decoded = decode::<i32>(&encoded).unwrap();

        assert_eq!(test, decoded)
    }

    #[test]
    fn test_encode_decode_i64_slice() {
        let mut rng = rand::thread_rng();
        let test: Vec<i64> = (0..10).map(|_| rng.gen()).collect();

        let encoded = encode_slice(&test);
        let decoded = decode::<i64>(&encoded).unwrap();

        assert_eq!(test, decoded)
    }

    #[test]
    fn test_encode_decode_i128_slice() {
        let mut rng = rand::thread_rng();
        let test: Vec<i128> = (0..10).map(|_| rng.gen()).collect();

        let encoded = encode_slice(&test);
        let decoded = decode::<i128>(&encoded).unwrap();

        assert_eq!(test, decoded)
    }
}
