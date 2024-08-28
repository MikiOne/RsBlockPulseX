// use thiserror::Error;
//
// #[derive(Error, Debug, Display)]
// pub enum Error {
//     RocksdbErr(#[from] rocksdb::Error),
// }
//！The error type for Indexer.
use thiserror::Error;

/// A list specifying general categories of Indexer error.
#[derive(Error, Debug)]
pub enum Error {
    /// Underlying DB error
    #[error("Db error {0}")]
    DB(String),
}

impl From<rocksdb::Error> for Error {
    fn from(e: rocksdb::Error) -> Error {
        Error::DB(e.to_string())
    }
}
