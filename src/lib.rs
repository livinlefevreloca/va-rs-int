mod varint_serde;
mod unsignedint;

pub fn encode_slice<I: varint_serde::VarIntEncode>(nums: &[I]) -> Vec<u8>
    where I: Sized
{
    nums.iter().flat_map(|n| n.encode()).collect()
}

pub fn encode<I: varint_serde::VarIntEncode>(num: I) -> Vec<u8>
    where I: Sized
{
    num.encode()
}


pub fn decode<V: varint_serde::VarIntDecode<I>, I>(bytes: V) -> Vec<I> {
    bytes.decode()
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_encode_u8() {
        let num1: u8 = 127;
        let num2: u8 = 255;
        
        let encoded1 =  encode(num1);
        let encoded2 = encode(num2);
        
        assert!(encoded1.len() == 1);
        assert!(encoded2.len() == 2);

        assert_eq!(&[127].as_slice(), &encoded1);
        assert_eq!(&[255, 1].as_slice(), &encoded2);
    }

    #[test]
    fn test_encode_u16() {
        let num1: u16 = 127;
        let num2: u16 = 255;
        let num3: u16 = 65535;
        
        let encoded1 =  encode(num1);
        let encoded2 = encode(num2);
        let encoded3 = encode(num3);
        
        assert!(encoded1.len() == 1);
        assert!(encoded2.len() == 2);
        assert!(encoded3.len() == 3);

        assert_eq!(&[127].as_slice(), &encoded1);
        assert_eq!(&[255, 1].as_slice(), &encoded2);
        assert_eq!(&[255, 255, 3].as_slice(), &encoded3);
    }
}
