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
        'p' => parse_print(itr),
        'r' => parse_render(itr),
        'k' => parse_kill(itr),
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
    let mut bff: Vec<u32> = Vec::new();
    let mut iters = inp.chars();
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
    Some(Commands::AppendStr(bff))
}
fn parse_kill(inp: std::str::Chars) -> Option<Commands> {
    let mut itr = inp.clone();
    if itr.next()? != ' ' {
        return None;
    }
    let strfm = itr.as_str();
    let strpfm = strfm.strip_prefix('0').unwrap_or(strfm);
    // use this ::: i64::from_str_radix
    match strpfm.strip_prefix('x') {
        Some(rest) =>
        // In hex
        {
            match u32::from_str_radix(rest, 16) {
                Ok(val) => Some(Commands::Kill { pos: val }),
                _ => None,
            }
        }
        None =>
        //In dec
        {
            match strfm.parse::<u32>() {
                Ok(val) => Some(Commands::Kill { pos: val }),
                Err(_) => None,
            }
        }
    }
    /*
    let appr = itr.next()?;
    if appr == 'x' || (appr == '0' && itr.next()? == 'x') {
        // do hexadecimal parsing
    }*/
}
fn parse_print(inp: std::str::Chars) -> Option<Commands> {
    let mut itr = inp.clone();
    match itr.next() {
        Some('x') => final_check(itr, Commands::Print(RawBase::Hex)),
        None => final_check(itr, Commands::Print(RawBase::Dec)),
        _ => None,
    }
}

fn parse_render(inp: std::str::Chars) -> Option<Commands> {
    let (is_not_basic, itr) = string_exact_check(inp, "32".chars());
    if !is_not_basic {
        return Some(Commands::Render(EncodingType::UTF8));
    }
    let (is_le, _) = string_exact_check(itr, "le".chars());
    Some(Commands::Render(if is_le {
        EncodingType::UTF32LE
    } else {
        EncodingType::UTF32
    }))
}
