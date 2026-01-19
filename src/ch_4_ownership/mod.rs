fn immutable_reference_example() {
  let mut v: Vec<i32> = vec![1, 2, 3];
  let num: &i32 = &v[2];

  // let x = v[1]; // allowed

  println!("Third element is {}", *num);
  println!("Again, the third element is {}", *num);
  println!("Again, the third element is {}", *num);

  v.push(4);
}

fn mutable_reference_example() {
  let mut v: Vec<i32> = vec![1, 2, 3];
  let num: &mut i32 = &mut v[2];

  // let x = v[1]; // not allowed

  *num += 1;

  println!("Third element is {}", *num);
  // println!("Vector is now {:?}", v);
}

pub fn main() {
  println!();
  println!("{}", "=".repeat(30) + "[IMMUTABLE REFERENCE EXAMPLE]");
  immutable_reference_example();
  println!("{}", "=".repeat(30) + "[MUTABLE REFERENCE EXAMPLE]");
  mutable_reference_example();
  println!();
}
