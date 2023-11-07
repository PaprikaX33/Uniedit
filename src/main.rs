mod cmd;
use std::io::{self, Write};
use unicode_normalization::UnicodeNormalization;

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
            cmd::command_list::Commands::Write { .. } => todo!(),
            cmd::command_list::Commands::Help => {
                println!("Help Page Here")
            }
            cmd::command_list::Commands::Compress => {
                vecbuff = vecbuff
                    .iter()
                    .map(|&code_point| char::from_u32(code_point).unwrap())
                    .nfc()
                    .map(|code_point| code_point as u32)
                    .collect();
            }
            cmd::command_list::Commands::Decompress => {
                vecbuff = vecbuff
                    .iter()
                    .map(|&code_point| char::from_u32(code_point).unwrap())
                    .nfd()
                    .map(|code_point| code_point as u32)
                    .collect();
            }
            cmd::command_list::Commands::InsertStr { .. } => todo!(),
            cmd::command_list::Commands::Modify { .. } => todo!(),
            cmd::command_list::Commands::Kill { pos: ps } => {
                let pos = ps as usize;
                if pos >= vecbuff.len() {
                    println!(
                        "Unable to remove element number {}, as buffer only contains {} {}",
                        pos,
                        vecbuff.len(),
                        if vecbuff.len() > 1 {
                            "elements"
                        } else {
                            "element"
                        }
                    );
                } else {
                    vecbuff.remove(pos);
                }
            }
            cmd::command_list::Commands::Render(_) => {
                println!("{:?}", render_buffer(&vecbuff))
            }
            cmd::command_list::Commands::Valid => {
                println!(
                    "{}!",
                    if render_buffer(&vecbuff).is_none() {
                        "Invalid"
                    } else {
                        "Valid"
                    }
                )
            }
        };
    }
}

fn render_buffer(vecbuff: &Vec<u32>) -> Option<Vec<char>> {
    //vecbuff.iter().map(|&x| char::from_u32(x)?)
    vecbuff.iter().try_fold(Vec::new(), |mut acc, &x| {
        acc.push(char::from_u32(x)?);
        Some(acc)
    })
}
