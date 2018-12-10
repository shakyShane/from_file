## from_file [![Build Status](https://travis-ci.org/shakyShane/from_file.svg?branch=master)](https://travis-ci.org/shakyShane/from_file)

> A simple convenience to deserialize a rust Struct or Enum directly from a file path.

It saves you from having to convert a string into a file-path, attempt to read the contents & then
deserialize. It's a wrapper around `serde` so you can use all of the features that you would normally 👍

**Links:** 

- [from_file on crates.io](https://crates.io/crates/from_file) 
- [from_file_derive on crates.io](https://crates.io/crates/from_file_derive)
- [docs](https://docs.rs/from_file/x/from_file/) 


## Example

```rust
#[derive(Deserialize, FromFile, Debug, PartialEq)]
struct Person {
    name: String,
    age: usize
}

// Now `Person` has a `from_file()` method that will read a file from
// disk and automatically attempt to deserialize it 👌
let p = Person::from_file("test/fixtures/person.json").expect("file -> Person");

println!("hey {}!", p.name);
```

### Full example with imports and error handing

```rust
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate from_file_derive;
extern crate from_file;

use from_file::FromFile;

#[derive(Deserialize, FromFile, Debug, PartialEq)]
struct Person {
    name: String,
    age: usize
}

fn main() {
    match Person::from_file("test/fixtures/person.json") {
        Ok(p) => println!("Got a Person from a file!"),
        Err(e) => eprintln!("{}", e)
    }
}
```
