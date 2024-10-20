enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    basic_enum_usage();
    store_value_in_enum();
    enum_can_implement_methods();
    the_builtin_option_enum();
    option_type_cannot_be_used_as_valid_value();
}

fn basic_enum_usage() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
}

fn store_value_in_enum() {
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
}

fn enum_can_implement_methods() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn the_builtin_option_enum() {
    let _some_number = Some(5); // Implicit type: Option<i32>
    let _some_char = Some('e'); // Implicit type: Option<char>

    let _absent_number: Option<i32> = None;
}

fn option_type_cannot_be_used_as_valid_value() {
    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);

    // let sum = x + y; -> would not compile because y could be `None`
}
