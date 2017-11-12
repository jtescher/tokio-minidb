extern crate bytes;
extern crate futures;
#[macro_use]
extern crate nom;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::collections::hash_map::Iter;
use std::io;
use std::sync::{Arc, Mutex};

pub mod server;
mod storage;
pub use storage::in_memory_db::InMemoryDB;

// Simple key value database interface
pub trait DB: Sized {
    fn new() -> Result<Self, io::Error>;
    fn get(&self, key: &String) -> DBResult;
    fn put(&mut self, key: String, value: String) -> DBResult;
    fn delete(&mut self, key: &String) -> DBResult;
    fn iter(&self) -> DBIterator;

    // Helper function to build thread safe reference
    fn handle(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    // Helper function to invoke a given command
    fn process_command(&mut self, command: DBCommand) -> DBResult {
        match command {
            DBCommand::Get(ref key) => self.get(key),
            DBCommand::Put(key, value) => self.put(key, value),
            DBCommand::Delete(ref key) => self.delete(key),
            DBCommand::Bad => Ok(Some("Invalid command".to_string())),
        }
    }
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

#[derive(Debug)]
pub enum DBCommand {
    Get(String),
    Put(String, String),
    Delete(String),
    Bad,
}

type DBResult = Result<Option<String>, io::Error>;
