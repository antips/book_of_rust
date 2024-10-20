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
