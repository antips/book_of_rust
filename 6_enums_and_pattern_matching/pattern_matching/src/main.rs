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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}
fn reroll() {}

fn main() {
    basic_pattern_matching();
    pattern_matching_with_enum_value();
    option_pattern_matching();
    match_default_case();
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

fn option_pattern_matching() {
    let five = Some(5);
    let _six = plus_one(five); // Returns Some(6)
    let _none = plus_one(None); // Returns None
}

fn match_default_case() {
    // Default case (and we need the value to use it)
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // Default case (and we don't need the value)
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // Default case (and we don't do anything)
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}
