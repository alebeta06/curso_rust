const X: i32 = 10;

fn main() {
    println!("{}", X);
    child();
}

fn child() {
    println!("{}", X);
}
