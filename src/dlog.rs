use std::rc::Rc;
use sha2::{Sha256, Digest};
use sha2::digest::{Update};
use crate::curve::{CURVE_ORDER, Point};
use crate::utils::{gen_u128_from_buf, generate_rand_num};

type CompactedPoint = Rc<Box<Point>>;

struct DLogProofField;

pub struct DLogProof {
    pub t: CompactedPoint,
    pub s: u128,
}

enum DlogFields {
    S(u128),
    T(CompactedPoint),
}

pub struct SerializedDlog(DlogFields, DlogFields);

impl DLogProof {
    fn new(t: CompactedPoint, s: u128) -> Self {
        Self { t, s }
    }

    fn hash_points(sid: &str, pid: u128, points: &[CompactedPoint]) -> u128 {
        let mut hasher = Sha256::new();

        Update::update(&mut hasher, sid.as_bytes());
        Update::update(&mut hasher, &pid.to_be_bytes());

        points.iter().for_each(|point| {
            Update::update(&mut hasher, &point.bytes());
        });

        let digest = hasher.finalize();

        gen_u128_from_buf(digest.as_slice())
    }

    pub fn prove(sid: &str, pid: u128, x: u128, y: CompactedPoint, base_point: CompactedPoint) -> Self {
        let r = generate_rand_num();
        let t = r * u128::from(**base_point);
        let c = DLogProof::hash_points(sid, pid, &[base_point, y, Rc::new(Box::new(t.into()))]);
        let s = (r + c * x) % CURVE_ORDER;

        DLogProof::new(Rc::new(Box::new(t.into())), s)
    }

    pub(crate) fn verify(&mut self, sid: &str, pid: u128, y: CompactedPoint, base_point: CompactedPoint) -> bool {
        let c = DLogProof::hash_points(sid, pid, &[base_point.clone(), y.clone(), self.t.clone()]);
        let lhs = self.s * u128::from(**base_point);
        let rhs =  u128::from(**self.t) + (u128::from(**y) * c);

        lhs == rhs
    }

    pub fn deserialize(serialized: SerializedDlog) -> Self {
        if let (DlogFields::T(t), DlogFields::S(s)) = (serialized.0, serialized.1) {
            return Self::new(t, s);
        }

        panic!("Incorrect items were passed");
    }

    pub fn serialize(&self) -> SerializedDlog {
        SerializedDlog(DlogFields::T(self.t.clone()), DlogFields::S(self.s))
    }

    pub fn to_string(&self) -> String {
        let t: String = (**self.t).into();
        let s = self.s.to_string();

        format!("{{ t:{t_val}, s:{s_val} }}", t_val = t, s_val = s)
    }
}

impl PartialEq for DLogProof {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl From<SerializedDlog> for DLogProof {
    fn from(serialized: SerializedDlog) -> Self {
        DLogProof::deserialize(serialized)
    }
}

impl DLogProofField {
    pub fn serialize(&self, dlog_proof: &DLogProof) -> SerializedDlog {
        dlog_proof.serialize()
    }

    pub fn deserialize(&self, serialized: SerializedDlog) -> DLogProof {
        serialized.into()
    }
}
