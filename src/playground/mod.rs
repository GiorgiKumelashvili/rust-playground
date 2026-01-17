pub fn main() {
    println!("{}", "=".repeat(30));

    let mut a = [0, 1, 2, 3];
    println!("{:?}", a);

    let (a_l, a_r) = a.split_at_mut(2);

    println!("{:?}", a_l);
    println!("{:?}", a_r);
}
