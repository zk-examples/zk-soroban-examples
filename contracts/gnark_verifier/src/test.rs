#![cfg(test)]

extern crate std;

use ark_bls12_381::{Fq, Fq2};
use ark_serialize::CanonicalSerialize;
use core::str::FromStr;
use serde::Deserialize;
use soroban_sdk::{
    Env, U256, Vec,
    crypto::bls12_381::{
        Bls12381Fr, Bls12381G1Affine, Bls12381G2Affine, G1_SERIALIZED_SIZE, G2_SERIALIZED_SIZE,
    },
};
use std::vec::Vec as AllocVec;

use crate::{Groth16Verifier, Groth16VerifierClient, Proof};

#[derive(Deserialize)]
struct ProofJson {
    pi_a: [std::string::String; 3],
    pi_b: [[std::string::String; 2]; 3],
    pi_c: [std::string::String; 3],
    #[serde(rename = "publicSignals")]
    public_signals: AllocVec<std::string::String>,
}

fn g1_from_coords(env: &Env, x: &str, y: &str) -> Bls12381G1Affine {
    let ark_g1 = ark_bls12_381::G1Affine::new(Fq::from_str(x).unwrap(), Fq::from_str(y).unwrap());
    let mut buf = [0u8; G1_SERIALIZED_SIZE];
    ark_g1.serialize_uncompressed(&mut buf[..]).unwrap();
    Bls12381G1Affine::from_array(env, &buf)
}

fn g2_from_coords(env: &Env, x1: &str, x2: &str, y1: &str, y2: &str) -> Bls12381G2Affine {
    let x = Fq2::new(Fq::from_str(x1).unwrap(), Fq::from_str(x2).unwrap());
    let y = Fq2::new(Fq::from_str(y1).unwrap(), Fq::from_str(y2).unwrap());
    let ark_g2 = ark_bls12_381::G2Affine::new(x, y);
    let mut buf = [0u8; G2_SERIALIZED_SIZE];
    ark_g2.serialize_uncompressed(&mut buf[..]).unwrap();
    Bls12381G2Affine::from_array(env, &buf)
}

fn create_client(e: &Env) -> Groth16VerifierClient<'_> {
    let contract_id = e.register(Groth16Verifier {}, ());
    Groth16VerifierClient::new(e, &contract_id)
}

#[test]
fn test() {
    let env = Env::default();

    let proof_json_str = include_str!("../../../data/gnark/proof.json");
    let proof_json: ProofJson = serde_json::from_str(proof_json_str).unwrap();

    let proof = Proof {
        a: g1_from_coords(&env, &proof_json.pi_a[0], &proof_json.pi_a[1]),
        b: g2_from_coords(
            &env,
            &proof_json.pi_b[0][0],
            &proof_json.pi_b[0][1],
            &proof_json.pi_b[1][0],
            &proof_json.pi_b[1][1],
        ),
        c: g1_from_coords(&env, &proof_json.pi_c[0], &proof_json.pi_c[1]),
    };

    let client = create_client(&env);

    let public_signal: u32 = proof_json.public_signals[0].parse().unwrap();
    let output = Vec::from_array(
        &env,
        [Bls12381Fr::from_u256(U256::from_u32(&env, public_signal))],
    );
    let res = client.verify_proof(&proof, &output);
    assert_eq!(res, true);

    let output = Vec::from_array(&env, [Bls12381Fr::from_u256(U256::from_u32(&env, 23))]);
    let res = client.verify_proof(&proof, &output);
    assert_eq!(res, false);
}
