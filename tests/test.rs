use vectordb::{loader::{rand_vec, create_db_random}};

#[test]
pub fn run_tests() {
    let v_len = 100;
    let limit = 10;
    let db = create_db_random(v_len, 1000);
    for _ in 0..1 {
        let query = rand_vec(v_len);
        let res_brute = db.query_brute(&query, limit);
        let res = db.query(&query, limit);
        assert_eq!(res, res_brute)
    }
    // println!("{:?}", res.iter().map(|(label, _vec, sim)| (*label, *sim)).collect::<Vec<(&String,Scalar)>>());
}
