
pub fn iterators() {

  let numbers = [1, 2, 3, 4];

  for x in numbers.iter() {
      println!("{}", x);
  }

  let mut c = Counter::new();
  c.next();
  let i = c.next();

  match i {
      Some(count) => println!("implement iterator {}", count),
      None => println!("nada")
  }
  
}

struct Counter {
  count: i32,
}

impl Counter {
  fn new() -> Counter {
      Counter { count: 0 }
  }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
      self.count += 1;
      Some(self.count)
    }
}