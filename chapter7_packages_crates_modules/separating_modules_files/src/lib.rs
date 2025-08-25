mod back_of_house;
mod front_of_house;

use front_of_house::hosting;

pub fn process_order() {
    // can import directory
    crate::back_of_house::preparing::create_meal();
    // can use "use" to use function
    // idiomatic to import module when using functions
    // idiomatic to import object directly when using structs, enums, etc.
    hosting::add_to_waitlist();
}
