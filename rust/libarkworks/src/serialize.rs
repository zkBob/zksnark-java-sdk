use ark_ec::short_weierstrass::{SWFlags, Affine, SWCurveConfig};
use ark_ff::{Fp12, Fp12Config, BigInteger256};
use ark_serialize::{Compress, CanonicalSerialize, Flags, CanonicalDeserialize, SerializationError};

const WORD_SIZE: usize = 32;

pub trait EthSerializeDeserialize {
    fn eth_serialize(&self) -> Vec<u8>;
    fn eth_deserialize(serialized: &[u8]) -> Result<Self, SerializationError> where Self: Sized;
    // this function doesn't perform point validation
    fn eth_deserialize_unchecked(serialized: &[u8]) -> Result<Self, SerializationError> where Self: Sized;
}

impl<P: SWCurveConfig> EthSerializeDeserialize for Affine<P> {
    fn eth_serialize(&self) -> Vec<u8> {
        if self.infinity {
            // infinity encoded as (0, 0)
            return vec![0; self.serialized_size(Compress::No)];
        }

        let mut serialized = vec![];
        self.serialize_with_mode(&mut serialized, Compress::No).expect("failed to serialize point");
        clear_flags(&mut serialized);
        convert_to_big_endian(&serialized)
    }

    fn eth_deserialize(serialized: &[u8]) -> Result<Self, SerializationError> {
        let mut serialized = convert_to_little_endian(serialized);
        restore_flags(&mut serialized);
        Self::deserialize_uncompressed(&serialized[..])
    }

    fn eth_deserialize_unchecked(serialized: &[u8]) -> Result<Self, SerializationError> {
        // It doesn't look correct, CHECK IT
        // Now it is necessary to produce the same behaviour as in the native implementation
        if serialized.len() == 0 || serialized.iter().all(|e| *e == 0) {
            return Ok(Affine::identity());
        }
        let mut serialized = convert_to_little_endian(serialized);
        restore_flags(&mut serialized);
        Self::deserialize_uncompressed_unchecked(&serialized[..])
    }
}

impl<P: Fp12Config> EthSerializeDeserialize for Fp12<P> {
    fn eth_serialize(&self) -> Vec<u8> {
        let mut serialized = vec![];
        self.serialize_with_mode(&mut serialized, Compress::No).expect("failed to serialize field element");
        convert_to_big_endian(&serialized)
    }

    fn eth_deserialize(serialized: &[u8]) -> Result<Self, SerializationError> {
        let serialized = convert_to_little_endian(serialized);
        Self::deserialize_uncompressed(&serialized[..])
    }

    fn eth_deserialize_unchecked(serialized: &[u8]) -> Result<Self, SerializationError> {
        Self::eth_deserialize(serialized)
    }
}

impl EthSerializeDeserialize for BigInteger256 {
    fn eth_serialize(&self) -> Vec<u8> {
        let mut serialized = vec![];
        self.serialize_with_mode(&mut serialized, Compress::No).expect("failed to serialize field element");
        convert_to_big_endian(&serialized)
    }

    fn eth_deserialize(serialized: &[u8]) -> Result<Self, SerializationError> {
        // It doesn't look correct, CHECK IT
        // Now it is necessary to produce the same behaviour as in the native implementation
        if serialized.len() == 0 {
            return Ok(BigInteger256::zero());
        }
        let serialized = convert_to_little_endian(serialized);
        Self::deserialize_uncompressed(&serialized[..])
    }

    fn eth_deserialize_unchecked(serialized: &[u8]) -> Result<Self, SerializationError> {
        Self::eth_deserialize(serialized)
    }
}

fn clear_flags(serialized: &mut Vec<u8>) {
    let last = serialized.len() - 1;
    serialized[last] &= 0b00111111;
}

fn restore_flags(serialized: &mut Vec<u8>) {
    let last = serialized.len() - 1;
    if is_zero_vec(serialized) {
        serialized[last] |= SWFlags::PointAtInfinity.u8_bitmask();
    }
}

fn is_zero_vec(vec: &Vec<u8>) -> bool {
    vec.iter().all(|b| *b == 0)
}

fn convert_to_little_endian(serialized: &[u8]) -> Vec<u8> {
    convert_to_big_endian(serialized)
}

fn convert_to_big_endian(serialized: &[u8]) -> Vec<u8> {
    serialized
        .chunks(WORD_SIZE)
        .map(|chunk| {
            let mut chunk = Vec::from(chunk);
            chunk.reverse();
            chunk
        })
        .fold(vec![], |mut acc, chunk| {
            acc.extend(chunk);
            acc
        })
}