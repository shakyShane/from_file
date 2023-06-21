#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate from_file_derive;
extern crate from_file;

use from_file::FromFile;

#[test]
fn test_derive() {
    #[derive(Deserialize, FromFile, Debug, PartialEq)]
    struct Person {
        name: String,
    }

    #[cfg(feature = "json")]
    {
        let p1 = Person::from_file("test/fixtures/person.json").expect("file->Person");
        assert_eq!(
            p1,
            Person {
                name: "Shane".into()
            }
        );
    }

    #[cfg(feature = "yaml")]
    {
        let p1 = Person::from_file("test/fixtures/person.yaml").expect("file->Person");
        assert_eq!(
            p1,
            Person {
                name: "Shane".into()
            }
        );
    }

    #[cfg(feature = "xml")]
    {
        let p1 = Person::from_file("test/fixtures/person.xml").expect("file->Person");
        assert_eq!(
            p1,
            Person {
                name: "Shane".into()
            }
        );
    }
}
