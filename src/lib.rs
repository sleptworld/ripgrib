use grib1::product::Grib1Product;
use std::{
    borrow::Borrow,
    ffi::OsString,
    fmt,
    fs::metadata,
    fs::File,
    io::{BufReader, Read},
    os::unix::prelude::MetadataExt,
    path::Path,
};

use crate::error::*;
mod config;
mod error;
mod grib1;
mod tools;

// Grib Version Branches
#[derive(Debug)]
pub enum Version {
    Grib1,
    Grib2,
}

// Section Trait
type FormattingMessage<T: AsRef<str>> = (usize, T);

trait Section: fmt::Display {
    fn parse<'a, T>(input: T) -> Self
    where
        T: Borrow<&'a [u8]>;
    // For printing more pretty
    fn formatting_print<T>(&self) -> Option<FormattingMessage<T>>
    where
        T: AsRef<str>;
}

trait PrFactory {
    fn make_pd<'a, 'b>(&'b self, version: Version, data: &'a [u8]) -> DataPack<'a>
    where
        'a: 'b;
}

pub struct PD;
impl PrFactory for PD {
    fn make_pd<'a, 'b>(&'b self, version: Version, data: &'a [u8]) -> DataPack<'a>
    where
        'a: 'b,
    {
        DataPack {
            _data: Raw {
                _raw_data: data,
                _length: data.len(),
                _read: 0,
            },
            _product: match version {
                Version::Grib1 => Box::new(Grib1Product::default()),
                _ => Box::new(Grib1Product::default()),
            },
        }
    }
}

pub struct DataPack<'a> {
    _data: Raw<'a>,
    _product: Box<dyn ProductRecord>,
}

#[derive(Debug)]
struct Raw<'a> {
    _raw_data: &'a [u8],
    _length: usize,
    _read: usize,
}

struct Header {
    keys: Vec<String>,
}

trait ProductRecord {
    fn header_parse(&mut self);
    fn data_parse(&mut self);
}

pub struct DataSet<'a>(Vec<DataPack<'a>>);

impl<'a> DataSet<'a> {}

///
/// File
///
pub struct ProductBundle<'a, T: AsRef<Path>> {
    _filename: OsString,
    _path: T,
    _raw_data: RawData,
    pub dataset: Option<DataSet<'a>>,
}

impl<'a, T: AsRef<Path>> ProductBundle<'a, T> {
    pub fn new(path: T) -> Result<Self> {
        let real_path = path.as_ref();
        if !(real_path.exists() && real_path.is_file()) {
            Err(UnexpectedError::FileNotExistError)
        } else {
            let name = real_path.file_name().unwrap();
            let file = File::open(path.as_ref())?;
            Ok(Ok(ProductBundle {
                _filename: OsString::from(name),
                _path: path,
                _raw_data: RawData::new(file),
                dataset: None,
            }))
        }
    }

    fn seek_grib(&mut self) {}
}

struct RawData {
    _file: File,
}

impl RawData {
    fn new(file: File) -> Self {
        RawData { _file: file }
    }

    fn read(&mut self) -> Result<()> {
        let file_size = self._file.metadata()?.len();

        let mini = 1024 * 1024 * 200;
        let middle = 1024 * 1024 * 2048;

        if file_size < mini {
            let mut raw: Vec<u8> = vec![];
            self._file.read_to_end(&mut raw)?;
        } else if file_size >= mini && file_size < middle {
        } else {
        }

        Ok(Ok(()))
    }

    fn seek<T: AsRef<[u8]>>(raw: T, pattern: T) {
        let p = pattern.as_ref();

        let bad_letter = {
            let len = p.len();
            let deltas = vec![len; len];

            let mut j = len - 1;

            while j > 0 {
                j -= 1;
            }
        };
    }
}
