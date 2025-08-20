fn main() {
    let rectangle1 = Rectangle {
        width: 10,
        height: 15,
    };

    let rectange1_area = area(&rectangle1);

    println!("rectange1_area: {}", rectange1_area);

    println!("rectangle1: {:#?}", rectangle1);
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
