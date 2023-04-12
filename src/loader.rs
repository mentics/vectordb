use lazy_static::lazy_static;
use rand::{distributions::Uniform, thread_rng, Rng};

use crate::{vectordb::{ItemEnc, Scalar, VectorDb, VKey}, create_db};

pub fn create_db_random(v_len: usize, data_count: usize) -> VectorDb<String> {
    let mut db = create_db::<String>(v_len);
    db.insert_all(load_data(v_len, data_count));
    return db;
}

pub fn load_data(v_len: usize, data_count: usize) -> Vec<ItemEnc<String>> {
    let mut items = Vec::new();

    for i in 0..data_count {
        items.push(rand_item(v_len, i.to_string()));
    }

    return items;
}

lazy_static! {
    static ref DISTRI: Uniform<Scalar> = Uniform::from(-1.0..1.0);
}

pub fn rand_vec(v_len: usize) -> VKey {
    let mut rng = thread_rng();
    let v:VKey = (&mut rng).sample_iter(*DISTRI).take(v_len).collect();
    return v;
}

pub fn rand_item(v_len: usize, val: String) -> ItemEnc<String> {
    let mut rng = thread_rng();
    let v:VKey = (&mut rng).sample_iter(*DISTRI).take(v_len).collect();
    return ItemEnc::new(v,val);
}