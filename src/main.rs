mod cmd;
use std::io::{self, Write};

fn stdreader() -> io::Result<String> {
    print!(">>");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;
    Ok(input)
}
fn main() {
    let mut vecbuff: Vec<u32> = Vec::new();
    println!("{:?}", cmd::capture("hello worlds"));
    while let Ok(inp) = stdreader() {
        let input = match cmd::capture(&inp) {
            Some(x) => x,
            None => {
                println!("Unknown command {}", inp.trim());
                continue;
            }
        };
        match input {
            cmd::Commands::Quit => break,
            cmd::Commands::Erase => vecbuff.clear(),
            cmd::Commands::AppendLit(val) => vecbuff.push(val),
            cmd::Commands::AppendStr(mut val) => vecbuff.append(&mut val),
            cmd::Commands::Print(base) => {
                match base {
                    cmd::RawBase::Dec => println!("{:?}", vecbuff),
                    cmd::RawBase::Hex => println!("{:02X?}", vecbuff),
                };
            }
            cmd::Commands::InsertLit { pos: ps, chr } => {
                let pos = ps as usize;
                if pos > vecbuff.len() {
                    vecbuff.push(chr);
                    continue;
                } else {
                    vecbuff.insert(pos, chr);
                }
            }
            _ => todo!(),
        };
    }
}
