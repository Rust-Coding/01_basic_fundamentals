
// Structs
struct User {
  username: String,
  email   : String,
  age     : i16,
  active  : bool,
  country : String,
}

impl User {
  fn birth (&self) -> i16 {
    let current_year = 2022;
    return current_year - self.age;
  }
}

// Tuple Struct
struct Color(i32, i32, i32);

pub fn structs_enums () {

  let user = User {
    username: String::from("Kevin"),
    email   : String::from("Kevin@gmail.com"),
    age     : 20,
    active  : true,
    country : String::from("USA"),
  };
  println!("{}", user.username);


  let user2 = new_user(String::from("Julio"), String::from("Julio@gmail.com"));
  println!("{}", user2.username);


  let user3 = User{
    username: String::from("Julio"),
    email   : String::from("Julio@gmail.com"),
    ..user2 // ..user2 is the same as user3 = user2
  };
  println!("{}", user3.username);

  let birth = user3.birth();
  println!("user 3 {}", birth);

  let red = Color(255, 0, 0);


  // Enums
  enum UserRoles {
    Admin,
    Moderator,
    User,
  }

  let user_type = UserRoles::Admin;

  // Enum with variants
  enum Message {
    Quit,
    SayHello(String),
  }

  let message = Message::SayHello(String::from("Hello"));

}

fn new_user ( nombre: String, email: String ) -> User {
  User {
    username: nombre,
    email,
    age     : 22,
    active  : true,
    country : String::from("Canada"),
  }
}