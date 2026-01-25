// example of exercising modules, which behaves exactly like a namespace and
// it would be similar to do it via folder structure
// for example add_to_waitlist() and seat_at_table() would be in
// file src/front_of_house/hosting/mod.rs or src/front_of_house/hosting.rs
mod test_folder;

fn deliver_order() {}

mod front_of_house {
  mod hosting {
    fn add_to_waitlist() {
      super::super::deliver_order(); // call parent method
    }

    fn seat_at_table() {}
  }

  pub mod serving {
    pub fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
  }

  pub mod serving2 {
    pub fn take_order() {}
  }

  #[derive(Debug)]
  pub struct Point(i32, pub i32);

  impl Point {
    pub fn origin() -> Self {
      Point(0, 0)
    }
  }

  #[derive(Debug)]
  pub struct CheckStruct2 {
    pub public: u32,
    private: u32
  }
}

#[derive(Debug)]
pub struct CheckStruct {
  pub public: u32,
  private: u32
}

pub fn eat_at_restaurant() {
  let some = CheckStruct {
    public: 1,
    private: 2
  };
  println!("{:#?}", some);

  /*
  - won't work because of 2 reasons
  - 1. private field we cannot access it
  - 2. if struct is missing field we cannot create struct
  */
  // let some = CheckStruct2 {
  //   public: 1
  // private: 2
  // };

  //  Absolute path is correct, will not work it must come from src/lib.rs
  // crate::front_of_house::hosting::add_to_waitlist();

  // Absolute path is correct, will not work it must come from src/lib.rs
  // crate::ch_7_package_crate_module::front_of_house::hosting::add_to_waitlist();

  // Relative path is correct, will not work because it is private
  // front_of_house::hosting::add_to_waitlist();

  // will work because evrything is public
  front_of_house::serving::take_order();

  // another example of struct
  let mut x = front_of_house::Point::origin();
  // x.0 += 1; // will not work because field is not public
  x.1 += 1;

  println!("{:#?}", x);

  {
    //* Adding use and a path in a scope is similar to creating a symbolic link in the filesystem
    // so we don't specify crate always we can use `use` keyword
    use crate::ch_7_package_crate_module::front_of_house::serving;

    serving::take_order();
    serving::take_order();

    //* use only creates the shortcut for the particular scope in which the use occurs
    mod customer {
      fn something() {
        // serving::take_order(); // will not work
      }
    }
  }
}

mod front_of_house2 {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

use front_of_house2::hosting;

pub fn eat_at_restaurant2() {
  hosting::add_to_waitlist();
}
