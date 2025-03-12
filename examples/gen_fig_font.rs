use figlet_rs::FIGfont;

fn main() {
    let small_font = FIGfont::from_file("assets/fonts/Cosmike.flf").unwrap();
    let figure = small_font.convert("Hello Rust");
    println!("{}", figure.unwrap());
}
