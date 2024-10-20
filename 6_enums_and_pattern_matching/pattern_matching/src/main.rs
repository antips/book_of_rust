#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    _Alaska,
    // --snip--
}

enum Coin {
    Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    basic_pattern_matching();
    pattern_matching_with_enum_value();
}

fn basic_pattern_matching() {
    let penny = Coin::Penny;
    let result = value_in_cents(penny);
    println!("The result value of Penny is {result}");
}

fn pattern_matching_with_enum_value() {
    let quarter = Coin::Quarter(UsState::Alabama);
    let result = value_in_cents(quarter);
    println!("The result value of Quarter is {result}");
}
