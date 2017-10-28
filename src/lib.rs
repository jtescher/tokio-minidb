use std::collections::hash_map::Iter;
use std::io;

mod storage;
pub use storage::in_memory_db::InMemoryDB;

// Simple key value database interface
pub trait DB: Sized {
    fn new() -> Result<Self, io::Error>;
    fn get(&self, key: &String) -> DBResult;
    fn put(&mut self, key: String, value: String) -> DBResult;
    fn delete(&mut self, key: &String) -> DBResult;
    fn iter(&self) -> DBIterator;
}

pub struct DBIterator<'a> {
    inner: Iter<'a, String, String>,
}

impl<'a> Iterator for DBIterator<'a> {
    type Item = (&'a String, &'a String);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

type DBResult = Result<Option<String>, io::Error>;
