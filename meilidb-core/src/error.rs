use std::{error, fmt, io};
use crate::serde::SerializerError;

pub type MResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    SchemaDiffer,
    SchemaMissing,
    WordIndexMissing,
    MissingDocumentId,
    Rkv(rkv::StoreError),
    Fst(fst::Error),
    RmpDecode(rmp_serde::decode::Error),
    RmpEncode(rmp_serde::encode::Error),
    Bincode(bincode::Error),
    Serializer(SerializerError),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<rkv::StoreError> for Error {
    fn from(error: rkv::StoreError) -> Error {
        Error::Rkv(error)
    }
}

impl From<fst::Error> for Error {
    fn from(error: fst::Error) -> Error {
        Error::Fst(error)
    }
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(error: rmp_serde::decode::Error) -> Error {
        Error::RmpDecode(error)
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(error: rmp_serde::encode::Error) -> Error {
        Error::RmpEncode(error)
    }
}

impl From<bincode::Error> for Error {
    fn from(error: bincode::Error) -> Error {
        Error::Bincode(error)
    }
}

impl From<SerializerError> for Error {
    fn from(error: SerializerError) -> Error {
        Error::Serializer(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match self {
            Io(e) => write!(f, "{}", e),
            SchemaDiffer => write!(f, "schemas differ"),
            SchemaMissing => write!(f, "this index does not have a schema"),
            WordIndexMissing => write!(f, "this index does not have a word index"),
            MissingDocumentId => write!(f, "document id is missing"),
            Rkv(e) => write!(f, "rkv error; {}", e),
            Fst(e) => write!(f, "fst error; {}", e),
            RmpDecode(e) => write!(f, "rmp decode error; {}", e),
            RmpEncode(e) => write!(f, "rmp encode error; {}", e),
            Bincode(e) => write!(f, "bincode error; {}", e),
            Serializer(e) => write!(f, "serializer error; {}", e),
        }
    }
}

impl error::Error for Error { }

