pub mod weird_option_info;

#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32)
}

impl Message {
  fn call(&self) {
    println!("Message: {:#?}", self);

    let is_self_write = matches!(self, Message::Write(_));
    let is_self_write_2 = match self {
      Message::Write(_) => true,
      _ => false
    };

    println!("is_self_write: {}", is_self_write);
    println!("is_self_write_2: {}", is_self_write_2);

    if let Message::Write(msg) = self {
      println!("is_self_write_3: {}", msg);
    }
  }
}

pub fn main() {
  let m = Message::Write(String::from("hello"));
  m.call();

  weird_option_info::main();
}
