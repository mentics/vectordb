use super::{Scalar, entries::VecData};

pub fn calc_mag(v1:&Vec<Scalar>) -> Scalar {
    let mut s = 0.0;
    for x in v1 {
        s += x;
    }
    return s.sqrt();
}

pub fn dot(v1:&VecData, v2:&VecData) -> Scalar {
    let mut s = 0.0;
    for i in 0..v1.len() {
        s += v1[i] * v2[i]
    }
    return s;
}

// [-1, 1], higher is more similar
pub fn similarity(v1:&VecData, v2:&VecData) -> Scalar {
    dot(v1, v2) / v1.mag() / v2.mag()
}
