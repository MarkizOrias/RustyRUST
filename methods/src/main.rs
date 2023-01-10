// Methods are similar to functions: we declare them with the fn keyword and a name,
// they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object, which we cover in Chapters 6 and 17, respectively),
// and their first parameter is always self, which represents the instance of the struct the method is being called on.

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        length: 50,
    };
    let rect2: Rectangle = Rectangle {
        width: 40,
        length: 60,
    };
    let rect3: Rectangle = Rectangle {
        width: 50,
        length: 70,
    };

    println!("Can rect1 fit rect2? {}", rect1.can_contain(&rect2));
    println!("Area of the rect3 is {}", rect3.area());
}

// Methods can be created for the structs (including functions, that return the given values).