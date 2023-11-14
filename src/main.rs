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
                let pos = ps;
                if pos > vecbuff.len() {
                    vecbuff.push(chr);
                    continue;
                } else {
                    vecbuff.insert(pos, chr);
                }
            }
            cmd::command_list::Commands::Write { enc, file } => match enc {
                cmd::command_list::EncodingType::UTF8 => {
                    let Some(utf_rep) = render_buffer(&vecbuff) else {
                        println!("The buffer is not in a valid state");
                        continue;
                    };
                    match std::fs::write(file.as_str(), utf_rep.iter().collect::<String>()) {
                        Ok(_) => (),
                        Err(_) => println!("Unable to write to file {}", file),
                    };
                }
                cmd::command_list::EncodingType::UTF32 => todo!(),
                cmd::command_list::EncodingType::UTF32LE => todo!(),
            },
            cmd::command_list::Commands::Read { file } => {
                vecbuff = match std::fs::read(file.as_str()) {
                    Ok(x) => x,
                    Err(_) => {
                        println!("Unable to open file {}", file);
                        continue;
                    }
                }
                .iter()
                .map(|x| *x as u32)
                .collect();
            }
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
            cmd::command_list::Commands::InsertStr { pos, txt } => {
                let pos = if pos > vecbuff.len() {
                    vecbuff.len()
                } else {
                    pos
                };

                let mut midbuffer = Vec::new();
                midbuffer.extend(&vecbuff[..pos]);
                midbuffer.extend(&txt);
                midbuffer.extend(&vecbuff[pos..]);
                vecbuff = midbuffer;
            }
            cmd::command_list::Commands::Modify { pos, chr } => {
                if pos >= vecbuff.len() {
                    println!(
                        "Unable to modify element number {}, as buffer only contains {} {}",
                        pos,
                        vecbuff.len(),
                        if vecbuff.len() > 1 {
                            "elements"
                        } else {
                            "element"
                        }
                    );
                } else {
                    vecbuff[pos] = chr;
                }
            }
            cmd::command_list::Commands::Kill { pos } => {
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
            cmd::command_list::Commands::Render(_) => match render_buffer(&vecbuff) {
                Some(x) => println!("{:?}", x),
                None => println!("Unable to render the buffer"),
            },
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
