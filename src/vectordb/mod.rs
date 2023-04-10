pub use self::vdb::VectorDb;

mod util;
mod vdb;
mod entries;

pub type Scalar = f32;

pub fn create_db<T>(len:usize) -> VectorDb<T> {
    return VectorDb::new(len);
}

pub struct ItemEnc<IT> {
    v: Vec<Scalar>,
    val: IT,
}
impl<IT> ItemEnc<IT> {
    pub fn new(v: Vec<Scalar>, val: IT) -> ItemEnc<IT> {
        return ItemEnc { v, val };
    }
}
