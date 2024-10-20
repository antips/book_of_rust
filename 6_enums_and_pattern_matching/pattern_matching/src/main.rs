enum Coin {
    Penny,
    _Nickel,
    _Dime,
    _Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::_Quarter => 25,
    }
}

fn main() {
    basic_pattern_matching();
}

fn basic_pattern_matching() {
    let penny = Coin::Penny;
    let result = value_in_cents(penny);
    println!("The result value is {result}");
}
