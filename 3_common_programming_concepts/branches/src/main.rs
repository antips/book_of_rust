fn main() {
    conditions_must_be_booleans();
    ternary();
}

fn conditions_must_be_booleans() {
    let nombre = 3;

    // Cannot be only "if nombre {":
    if nombre != 0 {
        println!("Le nombre valait autre chose que zéro");
    }
}

fn ternary() {
    let condition = true;
    let nombre = if condition { 5 } else { 6 };

    println!("La valeur du nombre est : {}", nombre);
}
