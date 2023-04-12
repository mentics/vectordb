use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vectordb::{vectordb::{VectorDb, VKey}, loader::{rand_vec, create_db_random}};

fn bench_brute(db: &mut VectorDb<String>, query: &VKey) {
    db.query_brute(&query, 10);
}

fn criterion_benchmark(c: &mut Criterion) {
    let key_len = 100;
    let mut db = create_db_random(key_len, 10000);

    let query = rand_vec(key_len);
    c.bench_function("brute force", |b| b.iter(
            || bench_brute(black_box(&mut db), black_box(&query))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);