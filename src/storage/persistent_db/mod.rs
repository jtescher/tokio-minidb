mod wal;

use bytes::BytesMut;
use std::io;
use std::io::BufRead;
use tokio_io::codec::Decoder;

use {DBIterator, DBResult, DB};
use server::LineCodec;
use storage::in_memory_db::InMemoryDB;
use self::wal::WAL;

pub struct PersistentDB {
    storage: InMemoryDB,
    wal: WAL,
}

impl PersistentDB {
    fn recover_from_wal(db: &mut InMemoryDB) -> Result<(), io::Error> {
        let wal = WAL::new()?;
        let mut codec = LineCodec;
        for command in wal.into_buf_reader().lines() {
            let mut command = BytesMut::from(command.unwrap_or("".to_string()));
            command.extend("\r\n".as_bytes());
            db.process_command(codec.decode(&mut command)?.unwrap())?;
        }
        Ok(())
    }
}

impl DB for PersistentDB {
    fn new() -> Result<PersistentDB, io::Error> {
        let mut storage = InMemoryDB::new()?;
        let wal = WAL::new()?;

        PersistentDB::recover_from_wal(&mut storage)?;

        Ok(PersistentDB { storage, wal })
    }

    fn put(&mut self, key: String, value: String) -> DBResult {
        self.wal
            .write_all(format!("put {} {}\n", key, value).as_bytes())
            .and_then(|_| self.storage.put(key, value))
    }

    fn get(&self, key: &String) -> DBResult {
        self.storage.get(key)
    }

    fn delete(&mut self, key: &String) -> DBResult {
        self.wal
            .write_all(format!("delete {}\n", key).as_bytes())
            .and_then(|_| self.storage.delete(key))
    }

    fn iter(&self) -> DBIterator {
        self.storage.iter()
    }
}
