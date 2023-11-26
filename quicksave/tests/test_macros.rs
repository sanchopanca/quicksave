#[test]
fn it_works() {
    use quicksave::save_slot;

    #[save_slot]
    struct Person {
        name: String,
        age: u8,
    }
}

// todo: require to use a constructor method
