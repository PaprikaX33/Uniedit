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

fn parse_print(inp: std::str::Chars) -> Option<Commands> {
    todo!();
}
