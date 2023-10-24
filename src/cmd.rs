/*! Parser for the input output command with the user
 */

pub mod command_list;
#[cfg(test)]
mod testing;
mod validator;

pub use command_list::*;
use validator::*;
/** Parse the user input

Parse the input string, and return the possible command to be executed.

# Example
```
let input = ".m20";
match capture(input) {...};
```
 */
pub fn capture(inp: &str) -> Option<Commands> {
    let trm = inp.trim();
    match trm.strip_prefix('.') {
        Some(cmd) => parse_cmd_selection(cmd.chars()), //Command
        None => parse_raw(trm),                        //Not a command
    }
}

/*
TODO : get the format for literal to use the hexadecimal aswell
*/
fn parse_cmd_selection(inp: std::str::Chars) -> Option<Commands> {
    let mut itr = inp.clone();
    match itr.next()?.to_lowercase().next()? {
        'q' => final_check(itr, Commands::Quit),
        '?' => final_check(itr, Commands::Help),
        'h' => final_check(itr, Commands::Help),
        'c' => final_check(itr, Commands::Compress),
        'd' => final_check(itr, Commands::Decompress),
        'e' => final_check(itr, Commands::Erase),
        'v' => final_check(itr, Commands::Valid),
        'm' => parse_modify(itr),
        'w' => parse_write(itr),
        'p' => parse_print(itr),
        'r' => parse_render(itr),
        'k' => parse_kill(itr),
        'i' => parse_insertion(itr),
        _ => parse_cmd_dec(inp),
    }
}
fn parse_cmd_dec(inp: std::str::Chars) -> Option<Commands> {
    match inp.as_str().parse::<u32>() {
        Ok(val) => Some(Commands::AppendLit(val)),
        Err(_) => None,
    }
}
fn parse_raw(inp: &str) -> Option<Commands> {
    Some(Commands::AppendStr(parse_raw_escapement(inp.chars())?))
}

fn parse_raw_escapement(inp: std::str::Chars) -> Option<Vec<u32>> {
    let mut bff: Vec<u32> = Vec::new();
    let mut iters = inp.clone();
    while let Some(chr) = iters.next() {
        match chr {
            '\\' => {
                let nxt = iters.next()?;
                bff.push(match nxt {
                    ' ' => ' ' as u32,
                    'n' => '\n' as u32,
                    't' => '\t' as u32,
                    '\\' => '\\' as u32,
                    '.' => '.' as u32,
                    _ => return None,
                })
            }
            _ => bff.push(chr as u32), // No escape
        }
    }
    Some(bff)
}
fn parse_kill(inp: std::str::Chars) -> Option<Commands> {
    let mut itr = inp.clone();
    if itr.next()? != ' ' {
        return None;
    }
    Some(Commands::Kill {
        pos: parse_number_value(itr)?,
    })
}

fn parse_number_value(inp: std::str::Chars) -> Option<u32> {
    let strfm = inp.as_str();
    let strpfm = strfm.strip_prefix('0').unwrap_or(strfm);
    // use this ::: i64::from_str_radix
    match strpfm.strip_prefix('x') {
        Some(rest) =>
        // In hex
        {
            match u32::from_str_radix(rest, 16) {
                Ok(val) => Some(val),
                _ => None,
            }
        }
        None =>
        //In dec
        {
            match strfm.parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        }
    }
}

fn parse_print(inp: std::str::Chars) -> Option<Commands> {
    let mut itr = inp.clone();
    match itr.next() {
        Some('x') => final_check(itr, Commands::Print(RawBase::Hex)),
        None => final_check(itr, Commands::Print(RawBase::Dec)),
        _ => None,
    }
}

fn parse_modify(inp: std::str::Chars) -> Option<Commands> {
    let (ps, cp) = inp.as_str().split_once(' ')?;
    let loc = parse_number_value(ps.chars())?;
    let cptrim = cp.strip_prefix('.').unwrap_or(cp);
    match cptrim.parse::<u32>() {
        Ok(val) => Some(Commands::Modify { pos: loc, chr: val }),
        Err(_) => None,
    }
}
fn parse_write(inp: std::str::Chars) -> Option<Commands> {
    let (is_32, itr) = string_exact_check(inp.clone(), "32".chars());
    if !is_32 {
        return parse_rear(itr.clone(), |fpath| {
            Some(Commands::Write {
                enc: EncodingType::UTF8,
                file: fpath.to_string(),
            })
        });
    }
    let (is_le, itr) = string_exact_check(itr, "le".chars());
    if !is_le {
        parse_rear(itr.clone(), |fpath| {
            Some(Commands::Write {
                enc: EncodingType::UTF32,
                file: fpath.to_string(),
            })
        })
    } else {
        parse_rear(itr.clone(), |fpath| {
            Some(Commands::Write {
                enc: EncodingType::UTF32LE,
                file: fpath.to_string(),
            })
        })
    }
}
fn parse_insertion(inp: std::str::Chars) -> Option<Commands> {
    let (ps, path) = inp.as_str().split_once(' ')?;
    let loc = parse_number_value(ps.chars())?;
    let mut startpol = path.chars();
    if startpol.next()? == '.' {
        // Literal mode
        match startpol.as_str().parse::<u32>() {
            Ok(val) => Some(Commands::InsertLit { pos: loc, chr: val }),
            Err(_) => None,
        }
    } else {
        // Non literal mode
        Some(Commands::InsertStr {
            pos: loc,
            txt: parse_raw_escapement(path.chars())?,
        })
    }
}

fn parse_render(inp: std::str::Chars) -> Option<Commands> {
    let (is_32, itr) = string_exact_check(inp.clone(), "32".chars());
    if !is_32 {
        return final_check(inp, Commands::Render(EncodingType::UTF8));
    }
    let (is_le, itr) = string_exact_check(itr, "le".chars());
    final_check(
        itr,
        Commands::Render(if is_le {
            EncodingType::UTF32LE
        } else {
            EncodingType::UTF32
        }),
    )
}

fn parse_rear<F>(inp: std::str::Chars, clos: F) -> Option<Commands>
where
    F: Fn(&str) -> Option<Commands>,
{
    let mut itr = inp;
    if itr.next()? != ' ' {
        return None;
    }
    return clos(itr.as_str());
}
