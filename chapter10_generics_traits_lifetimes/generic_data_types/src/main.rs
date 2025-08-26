use std::cmp::PartialOrd;

fn get_largest_in_list<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if let Some(first_item) = list.get(0) {
        let mut largest = first_item;

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        Some(largest)
    } else {
        None
    }
}

fn main() {
    let number_vector = vec![10, 70, 50, 100, 90];
    let largest_number = get_largest_in_list(&number_vector);

    let string_vector = vec![String::from("hello"), String::from("world")];
    // this line prevents compiling as doesn't fulfill PartialOrd
    // let largest_string = get_largest_in_list(string_vector);

    let point = Point { x: 5, y: 4 };
    println!("point x value is: {}", point.get_x());
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        return &self.x;
    }
}
