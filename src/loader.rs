use rand::{distributions::Uniform, thread_rng, Rng};

use crate::vectordb::{ItemEnc, Scalar};

pub fn load_data(v_len: usize, data_count: usize) -> Vec<ItemEnc<String>> {
    let distri = Uniform::from(-1.0..1.0);
    let mut rng = thread_rng();
    let mut items = Vec::new();

    for i in 0..data_count {
        let v:Vec<Scalar> = (&mut rng).sample_iter(distri).take(v_len).collect();
        items.push(ItemEnc::new(v,i.to_string()));
    }

    return items;
}