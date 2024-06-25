use thiserror::Error;

#[derive(Error)]
pub enum Error {
    RocksdbErr(#[from] rocksdb::Error),
}