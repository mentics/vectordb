use std::fmt::Debug;
pub use self::vdb::VectorDb;

mod util;
mod vdb;
mod entries;

pub type Scalar = f32;
pub type VKey = Vec<Scalar>;

pub fn create_db<T:Debug>(len:usize) -> VectorDb<T> {
    return VectorDb::new(len);
}

pub struct ItemEnc<IT> {
    v: VKey,
    val: IT,
}
impl<IT> ItemEnc<IT> {
    pub fn new(v: VKey, val: IT) -> ItemEnc<IT> {
        return ItemEnc { v, val };
    }
}

// #[derive(Debug)]
pub struct Result<'a, IT> {
    sim: Scalar,
    val: &'a IT,
    key: &'a VKey,
}
impl<IT> PartialEq for Result<'_, IT> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
// impl Eq for Result<IT> {
//     fn assert_receiver_is_total_eq(&self) {}
// }
impl<IT: Debug> Debug for Result<'_, IT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Result").field("sim", &self.sim).field("val", &self.val).finish()
    }
}