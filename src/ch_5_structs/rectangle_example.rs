#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn width(&self) -> bool {
    self.width > 0
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  fn this_is_static() {
    println!("hello from static")
  }
}

pub fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50
  };

  if rect1.width() {
    println!("The rectangle has a nonzero width; it is {}", rect1.width);
  }

  let rect2 = Rectangle {
    width: 10,
    height: 40
  };

  println!("rect1 is {:#?}", rect1);
  println!("rect2 is {:#?}", rect2);

  println!(
    "The area of the rectangle 1 is {} square pixels & rectangle 2 is {}.",
    rect1.area(),
    rect2.area()
  );

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

  Rectangle::this_is_static();
}
