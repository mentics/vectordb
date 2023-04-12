use std::ops::Index;
use super::{util::calc_mag, Scalar, VKey};

#[derive(Debug)]
pub struct VecData {
    pub v:VKey,
    mag:Scalar
}
impl VecData {
    pub fn new(v:VKey) -> Self {
        let m = calc_mag(&v);
        return VecData { v, mag: m };
    }

    // pub fn len(&self) -> usize {
    //     return self.v.len();
    // }

    pub fn mag(&self) -> Scalar {
        return self.mag;
    }

    pub fn key(&self) -> &VKey { &self.v }
}
impl Index<usize> for VecData {
    type Output = Scalar;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.v[index];
    }
}

pub struct ItemEncOpt<IT> {
    pub v: VecData,
    pub val: IT,
}
