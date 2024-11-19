use core::ops::Mul;
use elliptic_curve::{
    bigint::{ArrayEncoding, Encoding},
    sec1::{FromEncodedPoint, ToEncodedPoint},
    Curve, PrimeField,
};
use k256::{
    sha2::{Digest, Sha256},
    FieldBytes, ProjectivePoint, Scalar, Secp256k1, U256,
};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub(crate) struct DLogProof {
    pub t: ProjectivePoint,
    pub s: Scalar,
}

#[derive(Debug, Serialize, Deserialize)]
struct DLogProofSerde {
    t: Vec<u8>,
    s: Vec<u8>,
}

impl DLogProof {
    fn init(t: ProjectivePoint, s: Scalar) -> Self {
        Self { t, s }
    }

    #[allow(dead_code)]
    fn eq(&self, other: &DLogProof) -> bool {
        let self_dict = self.to_dict();
        let other_dict = other.to_dict();
        self_dict.t == other_dict.t && self_dict.s == other_dict.s
    }

    fn _hash_points(sid: &str, pid: Scalar, points: &[ProjectivePoint]) -> Scalar {
        let mut h = Sha256::new();

        h.update(sid.as_bytes());
        h.update(pid.to_bytes());
        for point in points {
            h.update(point.to_encoded_point(false).as_bytes());
        }
        let digest = h.finalize();
        Scalar::from_repr(digest).unwrap()
    }

    pub fn prove(
        sid: &str,
        pid: Scalar,
        x: Scalar,
        y: ProjectivePoint,
        base_point: Option<ProjectivePoint>,
    ) -> Self {
        // y = x * g
        let base_point = if let Some(base_point) = base_point {
            base_point
        } else {
            ProjectivePoint::GENERATOR
        };
        let r = crate::generate_random_number_r();
        let t = base_point.mul(r);
        let c = Self::_hash_points(sid, pid, &[base_point, y, t]);

        let q = Secp256k1::ORDER;
        let rcx: U256 = (r + c * x).into();
        let s = rcx.checked_rem(&q).unwrap();
        let s = Scalar::from_repr(s.to_be_byte_array()).unwrap();

        DLogProof::init(t, s)
    }

    pub fn verify(
        &self,
        sid: &str,
        pid: Scalar,
        y: ProjectivePoint,
        base_point: Option<ProjectivePoint>,
    ) -> bool {
        let base_point = if let Some(base_point) = base_point {
            base_point
        } else {
            ProjectivePoint::GENERATOR
        };
        let c = DLogProof::_hash_points(sid, pid, &[base_point, y, self.t]);
        let lhs = base_point.mul(Scalar::from(self.s));

        let rhs = self.t + (y.mul(c));
        lhs == rhs
    }

    fn to_dict(&self) -> DLogProofSerde {
        DLogProofSerde {
            t: self.t.to_encoded_point(false).as_bytes().to_vec(),
            s: self.s.to_bytes().to_vec(),
        }
    }

    #[allow(dead_code)]
    fn from_dict(t: Vec<u8>, s: Vec<u8>) -> Self {
        let t = ProjectivePoint::from_encoded_point(&t.as_slice().try_into().unwrap()).unwrap();
        let s = Scalar::from_repr(*FieldBytes::from_slice(s.as_slice())).unwrap();
        Self::init(t, s)
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        serde_json::to_string(&self.to_dict()).unwrap()
    }
}
