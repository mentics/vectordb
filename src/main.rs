use vectordb::{create_db, load_data};


fn main() {
    let mut db = create_db(10);
    let data = load_data(10, 100);
    db.insert_all(data);
    // test_queries(db);
}
