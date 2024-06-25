mod rocksdb;

use std::path::Path;
use crate::error::Error;

pub(crate) trait Store {
    type Batch: Batch;
    type Opts;

    fn new<P: AsRef<Path>>(opts: &Self::Opts, path: P) -> Result<Self, Error>;

    fn default_options() -> Self::Opts;

    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, Error>;

    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), Error>;

    fn exists<K: AsRef<[u8]>>(&self, key: K) -> Result<bool, Error>;

    // fn iter_from<K: AsRef<[u8]>>(&self, key: K) -> Result

    fn batch(&self) -> Self::Batch;
}

// RsBlockPulseX
pub(crate) trait Batch {
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&mut self, key: K, value: V);

    fn del<K: AsRef<[u8]>>(&mut self, key: K) -> Result<(), Error>;

    fn commit(&self) -> Result<(), Error>;
}