fn main() {
    assert_eq!(hello_world::english::hello(), "Hello!".to_string());
    assert_eq!(hello_world::english::greetings::hello(), "Hello!".to_string());
    assert_eq!(hello_world::english::farewells::goodbye(), "Goodbye!".to_string());

    println!("{}", hello_world::english::hello());
    println!("{}", hello_world::english::greetings::hello());
    println!("{}", hello_world::english::farewells::goodbye());
}
