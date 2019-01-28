
use rocksdb::DB;

static mut db: DB;


pub fn open_database(file_database: String) {
    db = DB::open_default(file_database).unwrap();
}


pub fn put_value(key: String, value: String) {
    db.put(key, value);
}

