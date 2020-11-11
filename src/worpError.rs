


// dont delete.  get this to work someday


// extern crate libc;
// extern crate librocksdb_sys as ffi;

// #[macro_use]
// mod ffi_util;
// mod util;

// pub mod backup;
// pub mod checkpoint;
// pub mod column_family;
// pub mod compaction_filter;
// mod comparator;
// mod db;
// mod db_iterator;
// mod db_options;
// mod db_pinnable_slice;
// mod db_vector;
// mod handle;
// pub mod merge_operator;
// mod open_raw;
// pub mod ops;
// mod read_only_db;
// mod slice_transform;
// mod snapshot;
// mod write_batch;

// pub mod prelude;

// pub use column_family::{ColumnFamily, ColumnFamilyDescriptor};
// pub use compaction_filter::Decision as CompactionDecision;
// pub use db::DB;
// pub use db_iterator::{DBIterator, DBRawIterator, Direction, IteratorMode};
// pub use db_options::{
//     BlockBasedIndexType, BlockBasedOptions, DBCompactionStyle, DBCompressionType, DBRecoveryMode,
//     MemtableFactory, Options, PlainTableFactoryOptions, ReadOptions, WriteOptions,
// };
// pub use db_pinnable_slice::DBPinnableSlice;
// pub use db_vector::DBVector;
// pub use read_only_db::ReadOnlyDB;
// pub use snapshot::Snapshot;
// pub use util::TemporaryDBPath;
// pub use write_batch::WriteBatch;


use std::error;
use std::fmt;

use std::boxed::Box;
use std::error::Error;

pub fn werr(msg: &str) -> Box<dyn Error> {
// pub fn werr(msg: &str) -> Result<(),Box<dyn Error>> {
  return Box::new(WError { message: msg.to_string() });
}

/// A simple wrapper round a string, used for errors reported from
/// ffi calls.
#[derive(Debug, Clone, PartialEq)]
pub struct WError {
    pub message: String,
}

impl WError {
    #[allow(dead_code)]
    fn new(message: String) -> WError {
        WError { message }
    }

    pub fn into_string(self) -> String {
        self.into()
    }
}

impl AsRef<str> for WError {
    fn as_ref(&self) -> &str {
        &self.message
    }
}

impl From<WError> for String {
    fn from(e: WError) -> String {
        e.message
    }
}


impl From<std::io::Error> for WError {
  fn from(error: std::io::Error) -> Self {
    WError::new(error.to_string())
  }
}


impl std::convert::From<warp::http::Error> for WError {
  fn from(error: warp::http::Error) -> Self {
    WError::new(error.to_string())
  }
}

impl warp::reject::Reject for WError {

}

impl std::convert::From<Box<dyn Error>> for WError {
  fn from(error: Box<dyn Error>) -> Self {
    WError::new(error.to_string())
  }}


// impl From<io::Error> for CliError {
//   fn from(error: io::Error) -> Self {
//       CliError::IoError(error)
//   }
// }


impl error::Error for WError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for WError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.message.fmt(formatter)
    }
}
