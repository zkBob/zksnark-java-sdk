use ark_bn254::{G1Affine, G2Affine, Bn254};
use ark_ec::{pairing::Pairing, CurveGroup, AffineRepr};
use ark_ff::{UniformRand, BigInteger256, Fp12, Field};

use crate::serialize::EthSerializeDeserialize;

pub fn random_g1() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    G1Affine::rand(&mut rng).eth_serialize()
}

pub fn random_g2() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    G2Affine::rand(&mut rng).eth_serialize()
}

pub fn g1_is_valid(serialized: Vec<u8>) -> bool {
    // if I understand correctly we can safely bypass is_in_correct_subgroup_assuming_on_curve check
    // since there is only one subgroup and we can't fail this check
    let g1 = G1Affine::eth_deserialize_unchecked(&serialized);
    if g1.is_err() {
        return false;
    }
    g1.unwrap().is_on_curve()
}

pub fn g2_is_valid(serialized: Vec<u8>) -> bool {
    // here we must check everything
    G2Affine::eth_deserialize(&serialized).is_ok()
}

pub fn add_g1(a_serialized: &[u8], b_serialized: &[u8]) -> Vec<u8> {
    let a = G1Affine::eth_deserialize_unchecked(a_serialized).unwrap();
    let b = G1Affine::eth_deserialize_unchecked(b_serialized).unwrap();
    (a + b).into_affine().eth_serialize()
}

pub fn mul_g1(a_serialized: &[u8], s_serialized: &[u8]) -> Vec<u8> {
    let p = G1Affine::eth_deserialize_unchecked(a_serialized).unwrap();
    let s = BigInteger256::eth_deserialize_unchecked(s_serialized).unwrap();
    p.mul_bigint(s).into_affine().eth_serialize()
}

pub fn multi_pairing(g1s_serialized: &[u8], g2s_serialized: &[u8], pairs: usize) -> Vec<u8> {
    let mut g1s = vec![];
    let mut g2s = vec![];
    for i in 0..pairs {
        g1s.push(G1Affine::eth_deserialize_unchecked(&g1s_serialized[i * g1_serialized_size()..(i+1) * g1_serialized_size()]).unwrap());
        g2s.push(G2Affine::eth_deserialize_unchecked(&g2s_serialized[i * g2_serialized_size()..(i+1) * g2_serialized_size()]).unwrap());
    }

    let result = Bn254::multi_pairing(g1s, g2s);
    result.0.eth_serialize()
}

pub fn pairing_check(g1s_serialized: &[u8], g2s_serialized: &[u8], pairs: usize) -> bool {
    let mut g1s = vec![];
    let mut g2s = vec![];
    for i in 0..pairs {
        g1s.push(G1Affine::eth_deserialize_unchecked(&g1s_serialized[i * g1_serialized_size()..(i+1) * g1_serialized_size()]).unwrap());
        g2s.push(G2Affine::eth_deserialize_unchecked(&g2s_serialized[i * g2_serialized_size()..(i+1) * g2_serialized_size()]).unwrap());
    }

    let result = Bn254::multi_pairing(g1s, g2s);
    result.0 == Fp12::ONE
}

pub fn g1_serialized_size() -> usize {
    // fix it
    64
}

pub fn g2_serialized_size() -> usize {
    // fix it
    128
}