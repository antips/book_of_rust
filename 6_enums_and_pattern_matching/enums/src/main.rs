enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    basic_enum_usage();
}

fn basic_enum_usage() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
}
