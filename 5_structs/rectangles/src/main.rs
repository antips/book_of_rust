#[derive(Debug)] // This Enables debug printing in the "println!" macro for the Rectangle struct:
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        // Associated function because no '&self' argument. Usage syntax: 'Rectagle::square(size)'
        // Can be used as constructors, like here:
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let square = Rectangle::square(60);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    debug_usage();
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
}

fn debug_usage() {
    let scale = 2;
    let rect_debug = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect_debug); // Pretty prints, but prints in STDERR instead and TAKES (and then RETURNS) the value OWNERSHIP

    println!("rect_debug is {rect_debug:?}"); // The {:?} format enables using the Debug trait to DEBUG the struct properties.
    println!("rect_debug is {rect_debug:#?}"); // The {:#?} format enables using the Debug trait to PRETTY PRINT the struct properties.
}
