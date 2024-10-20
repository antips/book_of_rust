enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum _AdvancedEnumMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    basic_enum_usage();
    store_value_in_enum();
}

fn basic_enum_usage() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
}

fn store_value_in_enum() {
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
}
