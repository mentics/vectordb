use rand::{thread_rng, distributions::Uniform, Rng};
use crate::vectordb::{VectorDb, Scalar};

pub fn run_tests() {
    let mut db = VectorDb::<&str>::new(10);
    let distri = Uniform::from(-1.0..1.0);
    let mut rng = thread_rng();
    let query:Vec<Scalar> = (&mut rng).sample_iter(distri).take(db.v_len).collect();
    let res = db.query(query, 10);
    println!("{:?}", res);
}