//Methods
// declared with a fn keyword and a name
//can have parameters and a return value
// defined within the context of a struct/enum/trait object
// first parameter is always self
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
//To define the function within the context of Rectangle, we start an impl block for Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
fn main() {
    println!("Hello, world!");
}
