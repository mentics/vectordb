use std::{cmp::min, ops::Range};

use super::{Scalar, entries::VecData, VKey};

pub fn calc_mag(v1:&VKey) -> Scalar {
    let mut s = 0.0;
    for x in v1 {
        s += x*x;
    }
    return s.sqrt();
}

pub fn dot(v1:&VKey, v2:&VKey) -> Scalar {
    let mut s = 0.0;
    for i in 0..v1.len() {
        s += v1[i] * v2[i]
    }
    return s;
}

pub fn dot_norm(v1:&VKey, v2:&VKey) -> Scalar {
    let mut s = 0.0;
    for i in 0..v1.len() {
        s += v1[i] * v2[i]
    }
    let m1 = calc_mag(v1);
    let m2 = calc_mag(v2);
    return s / m1 / m2;
}

// [-1, 1], higher is more similar
pub fn similarity(v1:&VecData, v2:&VecData) -> Scalar {
    dot(&v1.v, &v2.v) / v1.mag() / v2.mag()
}
// pub fn similarity2(vd1:&VecData, v2:&VKey, mag: Scalar) -> Scalar {
//     dot(&vd1.v, &v2) / vd1.mag() / mag
// }

// pub fn normalize(v:&mut VKey) {
//     let mag = calc_mag(v);
//     for i in 0..v.len() {
//         v[i] /= mag;
//     }
// }

// pub fn project<'a>(projections: &'a Vec<VKey>, v:&'a VKey) -> impl Iterator<Item = (usize,Scalar)> + 'a {
//     projections.iter().map(|p| dot_norm(p, v)).enumerate()
// }

pub fn range_around(ind: usize, dist: usize, mex: usize) -> Range<usize> {
    let left = if dist > ind { 0 } else { ind - dist };
    let right = min(mex, ind + dist);
    return left..right;
}


// pub fn proj_itr(input: &VKey, width: usize) -> impl Iterator<Item = Scalar> {
//     let mag = calc_mag(input);

//     let len = input.len();
//     let mut proj = 0.0;
//     let mut i = 0;
//     let mut ind_index = 0;
//     return input.iter().enumerate().map(|(i, x)| {
//         proj += x / mag;
//         if i % width  == 0 {
//             res.push(proj);
//             proj = 0.0;
//             ind_index += 1;
//         }
//     });
// }

pub fn proj_itr(input: &VKey, width: usize) -> Vec<Scalar> {
    // TODO: make into iterator to avoid allocation
    let mut res = Vec::new();
    let mag = calc_mag(input);

    let len = input.len();
    let mut proj = 0.0;
    let mut i = 0;
    // let mut ind_index = 0;
    while i < len {
        proj += input[i] / mag;
        i += 1;
        if i % width  == 0 {
            res.push(proj);
            proj = 0.0;
            // ind_index += 1;
        }
    }
    return res;
}