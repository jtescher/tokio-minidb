use std::collections::HashMap;
use std::io;

use {DBIterator, DBResult, DB};

#[derive(Debug)]
pub struct InMemoryDB {
    data: HashMap<String, String>,
}

impl DB for InMemoryDB {
    fn new() -> Result<InMemoryDB, io::Error> {
        Ok(InMemoryDB {
            data: HashMap::new(),
        })
    }

    fn get(&self, key: &String) -> DBResult {
        Ok(self.data.get(key).cloned())
    }

    fn put(&mut self, key: String, value: String) -> DBResult {
        Ok(self.data.insert(key, value))
    }

    fn delete(&mut self, key: &String) -> DBResult {
        Ok(self.data.remove(key))
    }

    fn iter(&self) -> DBIterator {
        DBIterator {
            inner: self.data.iter(),
        }
    }
}
