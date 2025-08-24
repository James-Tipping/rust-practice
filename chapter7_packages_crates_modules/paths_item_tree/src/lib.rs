mod front_of_house {
    // pub needed to access hosting module (container)
    pub mod hosting {
        // pub neded to access function - adding pub to a module only allows access to the module,
        // not what is contained within the module
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    // rust compiler way of obtaining deliver_order
    // use crate::deliver_order;

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub country: String,
        discounted: bool,
    }

    impl Breakfast {
        pub fn create_breakfast(country: &str) -> Breakfast {
            Breakfast {
                country: String::from(country),
                discounted: false,
            }
        }
    }
}

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

pub fn eat_breakfast() {
    let mut breakfast = crate::back_of_house::Breakfast::create_breakfast("English");

    breakfast.country = String::from("French");

    // cannot accesss as field is private
    // breakfast.discounted = true;
}
