mod cmd;

fn main() {
    println!("{:?}", cmd::capture(".ww"));
    println!("{:?}", cmd::capture("230"));
    println!("{:?}", cmd::capture("hello worlds"));
}
