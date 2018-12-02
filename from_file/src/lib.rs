//! This crate provides the trait [FromFile] that can be implemented or derived
//! for any struct or enum. Upon doing so, you'll get a `from_file` method
//! that allows you to skip having read the file the disk & choosing the correct
//! serde method - that will be done based on the file extension.
//!
//! # Quick Preview
//!
//! All examples require that serde Deserialize is also derived.
//! (see below for copy/paste example)
//!
//! ```
//! # #[macro_use]
//! # extern crate serde_derive;
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate from_file_derive;
//! # extern crate from_file;
//! # use from_file::FromFile;
//! #[derive(Deserialize)]
//! struct Person {
//!     name: String
//! }
//!
//! impl FromFile for Person {}
//!
//! fn main() {
//!     let path = "test/fixtures/person.json";
//!     let person = Person::from_file(path).expect("deserialize from file");
//!     assert_eq!(person.name, String::from("Shane"));
//! }
//! ```
//!
//! # Quick Preview with `from_file_derive`
//!
//! This requires the additional crate [from_file_derive](https://crates.io/crates/from_file_derive)
//!
//! ```
//! # #[macro_use]
//! # extern crate serde_derive;
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate from_file_derive;
//! # extern crate from_file;
//! # use from_file::FromFile;
//! #[derive(Deserialize, FromFile)]
//! struct Person {
//!     name: String
//! }
//!
//! fn main() {
//!     let path = "test/fixtures/person.json";
//!     let person = Person::from_file(path).expect("deserialize from file");
//!     assert_eq!(person.name, String::from("Shane"));
//! }
//! ```
//!
//! # Copy/Paste example
//!
//! ```
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde;
//!
//! #[macro_use]
//! extern crate from_file_derive;
//! extern crate from_file;
//!
//! use from_file::FromFile;
//!
//! #[derive(Deserialize, FromFile)]
//! struct Person {
//!     name: String
//! }
//!
//! fn main() {
//!     let path = "test/fixtures/person.json";
//!     let person = Person::from_file(path).expect("deserialize from file");
//!     assert_eq!(person.name, String::from("Shane"));
//! }
//! ```
//!
//! ### Full example with imports and error handing
//!
//! ```rust
//! #[macro_use]
//! extern crate serde_derive;
//!
//! #[macro_use]
//! extern crate from_file_derive;
//! extern crate from_file;
//!
//! use from_file::FromFile;
//!
//! #[derive(Deserialize, FromFile, Debug, PartialEq)]
//! struct Person {
//!     name: String,
//!     age: usize
//! }
//!
//! fn main() {
//!     match Person::from_file("test/fixtures/person.json") {
//!         Ok(p) => println!("Got a Person from a file!"),
//!         Err(e) => eprintln!("{}", e)
//!     }
//! }
//! ```
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FromFileError {
    InvalidExtension,
    InvalidInput,
    FileOpen(PathBuf),
    FileRead,
    SerdeError(String),
}

///
/// Implement this trait to enable your Struct's to be deserialized
/// from a file-path like
///
/// - conf/app.yaml
/// - file:conf/app.yaml
///
pub trait FromFile {
    ///
    /// Support serialising to .yml, .yaml & .json files by
    /// looking at the file extension and then choosing the correct
    /// serde method
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate serde_derive;
    /// # extern crate serde;
    /// # #[macro_use]
    /// # extern crate from_file_derive;
    /// # extern crate from_file;
    /// # use from_file::FromFile;
    /// #[derive(Deserialize)]
    /// struct Person {
    ///     name: String
    /// }
    ///
    /// impl FromFile for Person {}
    ///
    /// fn main() {
    ///     let path = "test/fixtures/person.json";
    ///     let person = Person::from_file(path).expect("deserialize from file");
    ///     assert_eq!(person.name, String::from("Shane"));
    /// }
    /// ```
    ///
    fn from_file(input: &str) -> Result<Self, FromFileError>
    where
        for<'de> Self: Deserialize<'de> + Sized,
    {
        let pb = PathBuf::from(input);
        let ext = pb
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or(FromFileError::InvalidExtension)?;
        match ext {
            "json" => <Self as FromFile>::from_json_file(input),
            "yml" | "yaml" => <Self as FromFile>::from_yml_file(input),
            _ => Err(FromFileError::InvalidExtension),
        }
    }

    ///
    /// From a string like `file:config.yaml`, try to read the file
    /// and if it exists, parse into a strongly typed struct `Self`
    ///
    fn from_yml_file(input: &str) -> Result<Self, FromFileError>
    where
        for<'de> Self: Deserialize<'de> + Sized,
    {
        <Self as FromFile>::get_file_path(input)
            .and_then(<Self as FromFile>::file_read)
            .and_then(<Self as FromFile>::from_yaml_string)
    }

    ///
    /// From a string like `file:config.yaml`, try to read the file
    /// and if it exists, parse into a strongly typed struct `Self`
    ///
    fn from_json_file(input: &str) -> Result<Self, FromFileError>
    where
        for<'de> Self: Deserialize<'de> + Sized,
    {
        <Self as FromFile>::get_file_path(input)
            .and_then(<Self as FromFile>::file_read)
            .and_then(<Self as FromFile>::from_json_string)
    }

    ///
    /// Parse strings like file:config.yaml to extract the file path only
    ///
    fn get_file_path(input: &str) -> Result<String, FromFileError> {
        let split: Vec<&str> = input.split(":").collect();
        match split.len() {
            1 => Ok(split[0].into()),
            2 => Ok(split[1].into()),
            _ => Err(FromFileError::InvalidInput),
        }
    }

    ///
    /// Attempt to Read the file's contents into a string
    ///
    fn file_read(input: String) -> Result<String, FromFileError> {
        let mut maybe_path = std::env::current_dir().expect("can read current dir");
        maybe_path.push(&input);
        let mut file = File::open(&input).map_err(|_| FromFileError::FileOpen(maybe_path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|_| FromFileError::FileRead)?;
        Ok(contents)
    }

    ///
    /// Parse any YAML string directly into a Self
    ///
    fn from_yaml_string(contents: String) -> Result<Self, FromFileError>
    where
        for<'de> Self: Deserialize<'de>,
    {
        serde_yaml::from_str(&contents).map_err(|e| FromFileError::SerdeError(e.to_string()))
    }

    ///
    /// Parse json string directly into a Self
    ///
    fn from_json_string(contents: String) -> Result<Self, FromFileError>
    where
        for<'de> Self: Deserialize<'de>,
    {
        serde_json::from_str(&contents).map_err(|e| FromFileError::SerdeError(e.to_string()))
    }
}

impl std::fmt::Display for FromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FromFileError::InvalidExtension => write!(f, "FromFileError::InvalidExtension"),
            FromFileError::InvalidInput => write!(f, "FromFileError::InvalidInput"),
            FromFileError::FileOpen(path) => {
                write!(f, "FromFileError::FileOpen - couldn't open {:?}", path)
            }
            FromFileError::FileRead => write!(f, "FromFileError::FileRead"),
            FromFileError::SerdeError(e) => write!(f, "FromFileError::SerdeError - {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FromFile;

    #[test]
    fn test_from_file() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Person {
            name: String,
        }
        impl FromFile for Person {}

        let p1 = Person::from_file("test/fixtures/person.json").expect("file->Person");
        assert_eq!(
            p1,
            Person {
                name: "Shane".into()
            }
        );

        let p1 = Person::from_file("test/fixtures/person.yaml").expect("file->Person");
        assert_eq!(
            p1,
            Person {
                name: "Shane".into()
            }
        );
    }
}
