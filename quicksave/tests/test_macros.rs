#[test]
fn it_works() {
    use quicksave::save_slot;

    #[save_slot]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person::new("John".to_string(), 30);

    person.save();
}

// todo: require to use a constructor method
