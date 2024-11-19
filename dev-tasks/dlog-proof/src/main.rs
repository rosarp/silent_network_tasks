/// Non-interactive Schnorr ZK DLOG Proof scheme with a Fiat-Shamir transformation
mod proof;

use elliptic_curve::{sec1::ToEncodedPoint, Field, PrimeField};
use k256::{elliptic_curve::rand_core::OsRng, ProjectivePoint, Scalar};
use num_bigint::BigUint;
use std::time::SystemTime;

fn generate_random_number() -> Scalar {
    Scalar::random(&mut OsRng)
}

fn scalar_to_biguint(scalar: &Scalar) -> BigUint {
    BigUint::from_bytes_be(&scalar.to_bytes().to_vec())
}

fn main() {
    let sid = "sid";
    let pid = Scalar::ONE;

    let x = generate_random_number();
    println!("x: {}", scalar_to_biguint(&x));

    let g = ProjectivePoint::GENERATOR;
    let y = g * x;

    let start_proof = SystemTime::now();

    let dlog_proof = proof::DLogProof::prove(sid, pid, x, y, None);

    println!(
        "Proof computation time: {} ms",
        start_proof.elapsed().unwrap().as_millis()
    );

    let encode_points = dlog_proof.t.to_encoded_point(false);
    println!(
        "Proof x: {} , y: {}",
        scalar_to_biguint(&Scalar::from_repr(*encode_points.x().unwrap()).unwrap()),
        scalar_to_biguint(&Scalar::from_repr(*encode_points.y().unwrap()).unwrap()),
    );
    println!("Proof s: {}", scalar_to_biguint(&dlog_proof.s));

    let start_verify = SystemTime::now();
    let result = dlog_proof.verify(sid, pid, y, None);
    println!(
        "Verify computation time: {} ms",
        start_verify.elapsed().unwrap().as_millis()
    );

    if result {
        println!("DLOG proof is correct");
    } else {
        println!("DLOG proof is not correct");
    }
}
