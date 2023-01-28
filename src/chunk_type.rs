use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidCharacter,
    InvalidLength(usize)
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct ChunkType([u8; 4]);

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, ChunkTypeError> {
        if value.iter().all(|byte|
            {byte.is_ascii_uppercase() || byte.is_ascii_lowercase()}) {
                Ok(Self(value))
        } else {
            Err(ChunkTypeError::InvalidCharacter)
        }
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, ChunkTypeError> {

        if s.len() != 4 {
            return Err(ChunkTypeError::InvalidLength(s.len()));
        }

        let bytes = s.as_bytes();

        if bytes.into_iter().all(|byte|
            {byte.is_ascii_uppercase() || byte.is_ascii_lowercase()}) {
                let mut chars: [u8; 4] = [0, 0, 0, 0];
                let mut i = 0;
                for b in bytes.into_iter() {
                    chars[i] = *b;
                    i += 1;
                }
                Ok(Self(chars))
        } else {
            Err(ChunkTypeError::InvalidCharacter)
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0 {
            write!(f, "{}", byte as char)?;
        }
        Ok(())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.0.iter().all(|byte| {byte.is_ascii_uppercase() || byte.is_ascii_lowercase()})
        && self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        (self.0[0] & 0x20) != 0x20
    }

    pub fn is_public(&self) -> bool {
        (self.0[1] & 0x20) != 0x20
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.0[2] & 0x20) != 0x20
    }

    pub fn is_safe_to_copy(&self) -> bool {
        (self.0[3] & 0x20) == 0x20
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}