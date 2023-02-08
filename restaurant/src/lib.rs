mod front_of_house; // brings into scope code from ./front_of_house.rs

pub use crate::front_of_house::hosting;

// Sibling to front_of_house module makes the module in scope
pub fn eat_at_restaurant() {
  hosting::add_to_waitlist(); // in scope from above pub use

  // front_of_house::hosting::add_to_waitlist(); // Relative path
}
