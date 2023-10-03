/*! Parser for the input output command with the user
 */

mod command_list;
#[cfg(test)]
mod testing;

use command_list::*;
//use regex::Regex;

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

fn parse_cmd_dec(inp: std::str::Chars) -> Option<Commands> {
    match inp.as_str().parse::<u32>() {
        Ok(val) => Some(Commands::AppendLit(val)),
        Err(_) => None,
    }
}

/** Validator check for the trailing input after the no argument command

The commands without any additional argument should consist of only
the command and the leading and trailing whitespace.
Additional character after the command is considered as invalid.

# Applicability
This check is valid for the following commmand:
+ `q` Commands::Quit
+ `h` Commands::Help
+ `?` Commands::Help
+ `c` Commands::Compress
+ `d` Commands::Decompress
+ `e` Commands::Erase
+ `v` Commands::Valid

 */
fn narg_val(text: &str, parsed: Commands) -> Option<Commands> {
    if text.is_empty() {
        Some(parsed)
    } else {
        None
    }
}
fn parse_cmd_selection(inp: std::str::Chars) -> Option<Commands> {
    let mut itr = inp.clone();
    match itr.next()? {
        'q' => narg_val(itr.as_str(), Commands::Quit),
        _ => parse_cmd_dec(inp),
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
                    _ => return None,
                })
            }
            _ => bff.push(chr as u32), // No escape
        }
    }
    Some(Commands::AppendStr(bff))
}
