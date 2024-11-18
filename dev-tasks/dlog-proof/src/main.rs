/// Non-interactive Schnorr ZK DLOG Proof scheme with a Fiat-Shamir transformation
mod proof;

use core::ops::Mul;
use elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
use k256::{ProjectivePoint, Scalar};
use rand::prelude::*;
use std::time::SystemTime;

pub fn generate_random_number() -> u64 {
    thread_rng().gen()
}

fn main() {
    let sid = "sid";
    let pid = 1;

    let x = generate_random_number();
    println!("x: {x}");

    let g = ProjectivePoint::GENERATOR;
    let y = g.mul(Scalar::from(x));

    let start_proof = SystemTime::now();

    let dlog_proof = proof::DLogProof::prove(sid, pid, x, y, None);

    println!(
        "Proof computation time: {} ms",
        start_proof.elapsed().unwrap().as_millis()
    );
    let encode_points = dlog_proof.t.to_encoded_point(false);
    println!(
        "Proof x: {:?} , y: {:?}",
        encode_points.x(),
        encode_points.y()
    );
    println!("Proof s: {}", dlog_proof.s);

    let start_verify = SystemTime::now();
    let is_valid = dlog_proof.verify(sid, pid, y, None);
    println!(
        "Verify computation time: {} ms",
        start_verify.elapsed().unwrap().as_millis()
    );

    if is_valid {
        println!("DLOG proof is correct");
    } else {
        println!("DLOG proof is not correct");
    }
}
