pub mod hosting; // declaration for hosting module, found at src/front_of_house/hosting.rs

mod serving { // another child module
  fn take_order() {}

  fn serve_order() {}

  fn take_payment() {}
}
