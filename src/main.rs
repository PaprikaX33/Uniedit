mod cmd;

fn main() {
    println!("Hello, world!");
    println!("{:?}", cmd::capture(".20"));
    println!("{:?}", cmd::capture(".ww"));
    println!("{:?}", cmd::capture("230"));
}
