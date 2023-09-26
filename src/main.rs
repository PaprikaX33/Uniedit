mod cmd;

fn main() {
    println!("Hello, world!");
    println!("{:?}", cmd::capture(".20"));
    println!("{:?}", cmd::capture(".ww"));
    println!("{:?}", cmd::capture("230"));
    println!("{:?}", cmd::capture("    .4294967297"));
    println!("{:?}", cmd::capture("    .4294967296  "));
    println!("{:?}", cmd::capture(".4294967295  "));
    println!("{:?}", cmd::capture("hello worlds"));
}
