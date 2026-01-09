//Methods
// declared with a fn keyword and a name
//can have parameters and a return value
// defined within the context of a struct/enum/trait object
// first parameter is always self
//methods can take ownership of self, borrow self immutably or mutably
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
//To define the function within the context of Rectangle, we start an impl block for Rectangle
//signature for area, we use &self instead of rectangle: &Rectangle
//&self is short for self: &Self
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    //rect1 argument
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
//method syntax to call area method on Rectangle instance
//method syntax goes after instance
//Add a dot followed by the method name, parentheses and any arguments
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
fn main() {
    println!("Hello, world!");
}
