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
            cmd::command_list::Commands::Write { enc, file } => {
                let Some(utf_rep) = render_buffer(&vecbuff) else {
                    println!("The buffer is not in a valid state");
                    continue;
                };
                match enc {
                    cmd::command_list::EncodingType::UTF8 => {
                        match std::fs::write(file.as_str(), utf_rep.iter().collect::<String>()) {
                            Ok(_) => (),
                            Err(_) => println!("Unable to write to file {}", file),
                        };
                    }
                    cmd::command_list::EncodingType::UTF32 => {
                        let mut splitted: Vec<u8> = bitsplitter(&vecbuff, false);
                        let mut comb: Vec<u8> = vec![0x00 as u8, 0x00, 0xFE, 0xFF];
                        comb.append(&mut splitted);
                        match std::fs::write(file.as_str(), comb) {
                            Ok(_) => (),
                            Err(_) => println!("Unable to write to file {}", file),
                        };
                    }
                    cmd::command_list::EncodingType::UTF32LE => {
                        let mut splitted: Vec<u8> = bitsplitter(&vecbuff, true);
                        let mut comb: Vec<u8> = vec![0xFF as u8, 0xFE, 0x00, 0x00];
                        comb.append(&mut splitted);
                        match std::fs::write(file.as_str(), comb) {
                            Ok(_) => (),
                            Err(_) => println!("Unable to write to file {}", file),
                        };
                    }
                };
            }
            cmd::command_list::Commands::Read { file } => {
                vecbuff = match std::fs::read_to_string(file.as_str()) {
                    Ok(x) => x,
                    Err(_) => {
                        println!("Unable to open file {}", file);
                        continue;
                    }
                }
                .chars()
                .map(|x| x as u32)
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

fn bitsplitter(inp: &Vec<u32>, big_endian: bool) -> Vec<u8> {
    inp.iter()
        .flat_map(if big_endian {
            |val| {
                vec![
                    ((val >> 24) as u32 & 0xff) as u8,
                    ((val >> 16) as u32 & 0xff) as u8,
                    ((val >> 8) as u32 & 0xff) as u8,
                    ((val >> 0) as u32 & 0xff) as u8,
                ]
            }
        } else {
            |val| {
                vec![
                    ((val >> 0) as u32 & 0xff) as u8,
                    ((val >> 8) as u32 & 0xff) as u8,
                    ((val >> 16) as u32 & 0xff) as u8,
                    ((val >> 24) as u32 & 0xff) as u8,
                ]
            }
        })
        .collect()
}
