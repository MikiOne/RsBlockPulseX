use std::path::Path;
use std::sync::Arc;
use rocksdb::{DB, Options, WriteBatch};
use crate::error::Error;
use crate::store::{Batch, Store};

struct RocksDBStore {
    db: Arc<DB>,
}

impl Store for RocksDBStore {
    type Batch = RocksDBBatch;
    type Opts = Options;

    fn new<P: AsRef<Path>>(opts: &Self::Opts, path: P) -> Result<Self, Error> {
        let db = Arc::new(DB::open(opts, path)?);
        Ok(RocksDBStore { db })
    }

    fn default_options() -> Self::Opts {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts
    }

    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, Error> {
        Ok(self.db.get(key)?)
    }

    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), Error> {
        Ok(self.db.put(key, value)?)
    }

    fn exists<K: AsRef<[u8]>>(&self, key: K) -> Result<bool, Error> {
        Ok(self.get(key)?.is_some())
    }

    fn batch(&self) -> Self::Batch {
        RocksDBBatch {
            db: Arc::clone(&self.db),
            wb: WriteBatch::default(),
        }
    }
}

struct RocksDBBatch {
    db: Arc<DB>,
    wb: WriteBatch,
}

impl Batch for RocksDBBatch {
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&mut self, key: K, value: V) {
        self.wb.put(key, value);
    }

    fn del<K: AsRef<[u8]>>(&mut self, key: K) -> Result<(), Error> {
        Ok(self.wb.delete(key)?)
    }

    fn commit(self) -> Result<(), Error> {
        Ok(self.db.write(self.wb)?)
    }
}