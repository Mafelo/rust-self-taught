#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        println!("The area of the rectangle 2 is {} square pixels.", self.width * self.height);
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width1 = 5;
    let height1 = 10;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area2(rect1));

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", area3(&rect1));
    println!("The dimensions of the rectangle is {:#?} sq pixels", rect1);
    rect1.area();

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
