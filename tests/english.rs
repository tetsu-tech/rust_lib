extern crate hello_world;

#[test]
fn test_greetings_hello() {
    assert_eq!(hello_world::english::greetings::hello(), "Hello!".to_string());
}

#[test]
fn test_farewells_goodbye() {
    assert_eq!(hello_world::english::farewells::goodbye(), "Goodbye!".to_string());
}
