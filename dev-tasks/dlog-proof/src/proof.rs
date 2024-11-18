use core::ops::Mul;
use elliptic_curve::{
    sec1::{FromEncodedPoint, ToEncodedPoint},
    Curve,
};
use k256::{
    sha2::{Digest, Sha256},
    ProjectivePoint, Scalar, Secp256k1,
};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub(crate) struct DLogProof {
    pub t: ProjectivePoint,
    pub s: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct DLogProofSerde {
    t: Vec<u8>,
    s: Vec<u8>,
}

impl DLogProof {
    fn init(t: ProjectivePoint, s: u64) -> Self {
        Self { t, s }
    }

    fn eq(&self, other: &DLogProof) -> bool {
        let self_dict = self.to_dict();
        let other_dict = other.to_dict();
        self_dict.t == other_dict.t && self_dict.s == other_dict.s
    }

    fn _hash_points(sid: &str, pid: i32, points: &[ProjectivePoint]) -> u64 {
        let mut h = Sha256::new();

        h.update(sid.as_bytes());
        h.update(pid.to_string().as_bytes());
        for point in points {
            h.update(point.to_encoded_point(false).as_bytes());
        }
        let digest = h.finalize();
        // TODO: Fix unwrap
        BigUint::from_bytes_be(&digest).to_u64().unwrap_or_default()
    }

    pub fn prove(
        sid: &str,
        pid: i32,
        x: u64,
        y: ProjectivePoint,
        base_point: Option<ProjectivePoint>,
    ) -> Self {
        // y = x * g
        let base_point = if let Some(base_point) = base_point {
            base_point
        } else {
            ProjectivePoint::GENERATOR
        };
        let r = crate::generate_random_number();
        let t = base_point.mul(Scalar::from(r));
        let c = Self::_hash_points(sid, pid, &[base_point, y, t]);

        let s = r + c * x;

        DLogProof::init(t, s)
    }

    pub fn verify(
        &self,
        sid: &str,
        pid: i32,
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
        // TODO: Fix unwrap
        let rhs = self.t + (y.mul(Scalar::from(c.to_u64().unwrap_or_default())));
        lhs == rhs
    }

    fn to_dict(&self) -> DLogProofSerde {
        DLogProofSerde {
            t: self.t.to_encoded_point(false).as_bytes().to_vec(),
            s: self.s.to_string().into_bytes(),
        }
    }

    fn from_dict(t: Vec<u8>, s: Vec<u8>) -> Self {
        let t = ProjectivePoint::from_encoded_point(&t.as_slice().try_into().unwrap()).unwrap();
        let s = u64::from_le_bytes(s.as_slice().try_into().unwrap());
        Self::init(t, s)
    }

    fn to_string(&self) -> String {
        serde_json::to_string(&self.to_dict()).unwrap()
    }
}
