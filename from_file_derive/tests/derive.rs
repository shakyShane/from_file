#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate from_file_derive;
extern crate from_file;

use from_file::FromFile;

#[test]
fn test_derive() {
    #[derive(Deserialize, FromFile)]
    struct Person {
        name: String,
    }

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
