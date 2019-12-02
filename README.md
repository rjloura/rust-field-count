# Field Count

This crate allows you to derive `FieldCount` on your structures.  This
implementation comes from a [post](https://stackoverflow.com/questions/54177438/how-to-programmatically-get-the-number-of-fields-of-a-struct) on stackoverflow that I put into a crate for my own use.

## Usage:
```
use field_count::FieldCount;

#[allow(dead_code)]
enum States {
    NewHampshire,
    EverywhereElse,
}

#[derive(FieldCount)]
#[allow(dead_code)]
struct TestAddress {
    number: u32,
    street: String,
    city: String,
    state: States,
}

#[derive(FieldCount)]
#[allow(dead_code)]
struct TestPerson {
    name: String,
    age: u16,
    address: TestAddress,
    offspring: Vec<TestPerson>,
}

#[test]
fn basic() {
    assert_eq!(TestPerson::field_count(), 4);
    assert_eq!(TestAddress::field_count(), 4);
    assert_ne!(TestPerson::field_count(), 14);
    assert_ne!(TestAddress::field_count(), 22);
}
```
