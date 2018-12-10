//! This macro enables `derive(FromFile)`, it should be used alongside
//! [from_file](https://crates.io/crates/from_file)
//!
//! # Example
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

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(FromFile)]
pub fn from_file_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_from_file_macro(&ast)
}

fn impl_from_file_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl FromFile for #name {}
    };
    gen.into()
}
