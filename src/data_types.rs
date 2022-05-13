
pub fn data_types() {

  let negative_number: i8 = -20;
  let positive_number: u8 = 20;

  let floating_point_number: f32 = 3.14;

  let boolean: bool = true;

  let character: char = 'a';

  let string: String = String::from("Hello, world!");
  let string_literal: &str = "Hello, world!";

  // Tuples
  let tuple: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tuple; // assign each value of tuple
  let value1 = tuple.0; // get first value of tuple

  // Arrays
  let array: [i32; 5] = [1, 2, 3, 4, 5];


}