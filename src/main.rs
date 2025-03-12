use std::io;
use std::path::Path;

use figlet_rs::FIGfont;

fn generate_with_font() {
    let small_font = FIGfont::from_file("fonts/Cosmike.flf").unwrap();
    let figure = small_font.convert("Hello Rust");
    println!("{}", figure.unwrap());
}

fn main() {
    let project_root = std::env::current_dir().unwrap();
    let current_file = file!();
    let full_path = project_root.join(current_file); // Store in a variable
    let current_dir = full_path.parent().unwrap(); // Now safe to borrow

    println!("Current directory: {}", current_dir.display());

    // let fontPath = Path::new(&current_dir).join("Cosmike.flf");
    // println!("==========================================");
    // println!("{}", fontPath.display());
}
