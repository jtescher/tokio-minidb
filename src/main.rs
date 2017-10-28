extern crate tokio_minidb;

use tokio_minidb::{InMemoryDB, DB};

fn main() {
    // Create new database.
    let mut db = InMemoryDB::new().unwrap();

    // Create some data. In this case store user data as JSON.
    let key = &"user-1".to_string();
    let value = &"{ \"first_name\": \"John\", \"last_name\": \"Doe\"}".to_string();

    // When we get a key that is not yet set it should return None.
    println!("Get {} => {:?}", key, db.get(key).unwrap());

    // When we add data it should save the data and return what was there before, in this case None.
    println!("Put {} => {}", key, value);
    db.put(key.clone(), value.clone()).unwrap();

    // Now when we get the data it should return our value that we associated with this key.
    println!("Get {} => {}", key, db.get(key).unwrap().unwrap());

    // We should be able to iterate over all keys and values in the store.
    println!("Iterate over keys...");
    for (k, v) in db.iter() {
        println!("Key: {}, Value: {}", k, v);
    }

    // If we delete data it should return the value that was stored at the key.
    println!("Delete key {}", key);
    db.delete(&key).unwrap();

    // And finally we should see our empty database so we know the delete was successful.
    println!("Final db: {:?}", db);
}
