pub mod rectangle_example;
pub mod rectangle_example2;

#[derive(Debug, Clone)]
struct User<'a> {
  name: String,
  email: String,
  age: u8,
  fav_color: &'a str
}

struct Empty;

pub fn main() {
  let user = User {
    name: String::from("John Doe"),
    email: String::from("Nt0fW@example.com"),
    fav_color: "Blue",
    age: 8
  };

  let user2 = User {
    age: 12,
    ..user.clone()
  };

  // println!("{:#?}", user);
  // println!("{:#?}", user2);

  //
  #[derive(Debug)]
  struct Point {
    x: i32,
    y: i32
  }

  let mut p = Point {
    x: 0,
    y: 0
  };

  let x = &mut p.x; // Borrow field x mutably

  p.y += 1;

  // cannot do this here because p has been moved p and p.y was borrowed
  // p = Point {
  //   x: 1,
  //   y: 0
  // };
  *x += 1; // Modify through borrow

  p = Point {
    // x: dbg!(1),
    x: 1,
    y: 0
  };

  // println!("{}, {}", p.x, p.y); // p.x is now 1, p.y is 0
  // dbg!("{}, {}", p.x, p.y);

  struct Somethin {
    name: String,
    age: i32
  }

  impl Somethin {
    fn update_age(&mut self, new_age: i32) {
      self.age = new_age;
    }

    fn get_age(&self) -> i32 {
      self.age
    }

    fn get_name(&self) -> &String {
      &self.name
    }
  }

  let mut something = Somethin {
    name: String::from("John Doe"),
    age: 8
  };

  something.update_age(23);

  let another_age = something.get_age() * 2;
  let name = something.get_name();

  println!("my name is {} and my age is {}", name, another_age);

  println!("{}", "=".repeat(30));

  rectangle_example::main();

  println!("{}", "=".repeat(30));

  rectangle_example2::main();
}
