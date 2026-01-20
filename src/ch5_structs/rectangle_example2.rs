struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn area(&self) -> u32 {
    // &self = immutable reference (read-only access)
    self.width * self.height
  }

  fn set_width(&mut self, width: u32) {
    // &mut self = mutable reference (write access needed!)
    self.width = width;
  }

  fn max(self, other: Self) -> Self {
    // self = takes ownership (consumes the rectangle)
    let w = self.width.max(other.width);
    let h = self.height.max(other.height);
    Rectangle {
      width: w,
      height: h
    }
  }
}

fn main_with_issue() {
  let mut rect = Rectangle {
    // rect is mutable - can call methods needing &mut self
    width: 0,
    height: 0
  };

  rect.set_width(1); // ✅ WORKS: rect is mut, so &mut rect is created automatically

  let rect_ref = &rect; // ❌ PROBLEM: Creates IMMUTABLE reference &Rectangle (read-only)

  // rect_ref.set_width(2); // ❌ COMPILE ERROR!
  // Why? set_width needs &mut self, but rect_ref is &self (immutable)
  // Rust rule: Cannot mutate through immutable reference!
  // Mutable and immutable borrows cannot coexist.
}

pub fn main() {
  let mut rect = Rectangle {
    width: 0,
    height: 0
  };
  rect.set_width(1);

  // Fix 1: Create MUTABLE reference
  let mut rect_ref = &mut rect; // &mut rect = mutable reference
  rect_ref.set_width(2); // ✅ WORKS: &mut self matches &mut rect_ref

  // Fix 2: Limit immutable borrow scope
  {
    let rect_ref = &rect; // Immutable borrow ends here
    println!("width: {}", rect_ref.area()); // Use read-only access
  } // rect_ref dropped, mutable access allowed again
  rect.set_width(3); // ✅ WORKS: no active borrows
}
