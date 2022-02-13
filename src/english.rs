// hello_world::english::hello の定義
pub fn hello() -> String {
    "Hello!".to_string()
}

// 子モジュール hello_world::english::greetings の宣言
pub mod greetings;
// 子モジュール hello_world::english::farewells の宣言
pub mod farewells;
