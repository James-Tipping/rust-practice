fn main() {
    let rectangle1 = Rectangle {
        width: 10,
        height: 15,
    };

    let rectange1_area = area(&rectangle1);

    println!("rectange1_area: {}", rectange1_area);

    println!("rectangle1: {:#?}", rectangle1);

    println!("rectange1 native area method: {}", rectangle1.area());
    println!("rectange1 native area method: {}", rectangle1.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // & is needed in the method or ownership of struct will change
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}
