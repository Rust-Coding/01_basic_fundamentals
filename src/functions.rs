
pub fn functions() {

    say_hello();
    return_age();
    return_age2();
    values_for_references("Kevin");

    let age =values_for_references2(&20);
    println!("Age: {}", age);

}

fn say_hello() {  
  println!("Hello, world!");
}

fn return_age() -> i8 {
  return 10;
}

fn return_age2() -> i8 {
  10
}

fn values_for_references( name : &str ) {
  println!("Hi {}", name);
}

fn values_for_references2( age : &i8 ) -> i8 {
  *age + 5
}