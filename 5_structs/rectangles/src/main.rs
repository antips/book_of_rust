#[derive(Debug)] // This Enables debug printing in the "println!" macro for the Rectangle struct:
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {rect1:?}"); // The {:?} format enables using the Debug trait to DEBUG the struct properties.
    println!("rect1 is {rect1:#?}"); // The {:#?} format enables using the Debug trait to PRETTY PRINT the struct properties.

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2); // Pretty prints, but prints in STDERR instead and TAKES (and then RETURNS) the value OWNERSHIP
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
