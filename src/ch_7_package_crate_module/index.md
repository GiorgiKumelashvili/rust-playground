# A path can take two forms:

- An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
- A relative path starts from the current module and uses self, super, or an identifier in the current module.

# Example of re-export - pub use vs use - Complete Comparison

`pub use` re-exports items publicly, while plain `use` keeps them private to the current module.

## Version 1: Without `pub use` (Private Import Only)
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

// ❌ PRIVATE import - only works INSIDE this file
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // ✅ WORKS: local shortcut inside this module
    hosting::add_to_waitlist();
}

// ❌ ERROR: external code CANNOT see 'hosting'
```

### External users must use verbose path:
```rust
// In another file/crate:
crate::front_of_house::hosting::add_to_waitlist(); // Verbose!
```

### Version 2: With pub use (Public Re-export)
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

// ✅ PUBLIC re-export - works locally AND externally
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // ✅ WORKS: local shortcut (same as before)
    hosting::add_to_waitlist();
}
```

### External users get clean API:
```rust
// In another file/crate:
your_crate::hosting::add_to_waitlist(); // Clean! No deep nesting
```

### What You Can Do
| Scenario                      | use only                              | pub use                      |
| ----------------------------- | ------------------------------------- | ---------------------------- |
| Use inside same file          | ✅ hosting::add_to_waitlist()          | ✅ hosting::add_to_waitlist() |
| External crates call your API | ❌ crate::front_of_house::hosting::... | ✅ crate::hosting::...        |
| Library users get clean paths | ❌ Verbose paths                       | ✅ Flat public API            |