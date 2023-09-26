/*! Parser for the input output command with the user
 */

#[cfg(test)]
mod testing;
//use regex::Regex;

/** Encoding type for the writing and rendering
 */
#[derive(Debug, Eq, PartialEq)]
#[allow(unused_variables)]
#[allow(dead_code)]
pub enum EncodingType {
    /// As UTF-8
    UTF8,
    /// As UTF-32 Big Endian
    UTF32,
    /// As UTF-32 Little Endian
    UTF32LE,
}

/** Base number for the printing command

The buffer of the text can be printed in the hex form, or decimal form.
This enum is a tag for whether to print it as a hex form or decimal.
 */
#[derive(Debug, Eq, PartialEq)]
#[allow(unused_variables)]
#[allow(dead_code)]
pub enum RawBase {
    /// Decimal type
    Dec,
    /// Hexadecimal type
    Hex,
}

/**
Command entered by the user

A tagged union of the command entered by the user, the corresponding argument,
and optional information required to execute the command.

Some commands are having optional argument, and the parameter can be obtained
by asking the user about the parameter.
 */
#[derive(Debug, Eq, PartialEq)]
#[allow(unused_variables)]
#[allow(dead_code)]
pub enum Commands {
    /// Quit the program
    /// # Command
    /// + `.q`
    Quit,
    /// Write the buffer to a file
    /// # Command
    /// + `.w <file>` : Write as UTF-8 to *file*.
    /// + `.w32 <file>` : Write as UTF-32 Big Endian to *file*.
    /// + `.w32LE <file>` : Write as UTF-32 Little Endian to *file*.
    Write {
        enc: EncodingType,
        file: String,
    },
    /// The help page of the program
    /// Should print all of the available command and the usage
    /// # Command
    /// + `.h`
    /// + `.?`
    Help,
    /// Compress the text in the buffer to NFC representation
    /// # Command
    /// + `.c`
    Compress,
    /// Decompress the text in the buffer to NFD representation
    /// # Command
    /// + `.d`
    Decompress,
    /// Append literal unicode codepoint to the buffer
    /// # Command
    /// + `.ddd`
    /// + `.0xnnn`
    /// + `.xnnn`
    /// Where `d` is decimal digit, and `n` is hexadecimal digit
    AppendLit(u32),
    /// Append the string inserted string to the buffer
    /// # Note
    /// all input that is not prefixed with `.` is considered as the raw append string
    AppendStr(Vec<u32>),
    InsertLit {
        pos: u32,
        chr: u32,
    },
    InsertStr {
        pos: u32,
        txt: Vec<u32>,
    },
    Modify {
        pos: u32,
        chr: u32,
    },
    /// Kill or delete a character from the stream
    /// # Note
    /// All stream following the removed character is pushed forward
    Kill {
        pos: u32,
    },
    Print(RawBase),
    /// Purge the buffer to empty it
    /// # Command
    /// + `.e`
    Erase,
    /// Render the buffer to the stdout
    /// # Note
    /// defaulted to UTF-8
    /// # Command
    /// `.r` : Render to stdout as UTF-8
    /// `.r32` : Render to stdout as UTF-32 Big Endian
    /// `.r32LE` : Render to stdout as UTF-32 Little Endian
    Render(EncodingType),
    /// Validate the current buffer
    /// # Command
    /// `.v`
    Valid,
}

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
fn parse_cmd_selection(inp: std::str::Chars) -> Option<Commands> {
    let mut itr = inp.clone();
    match itr.next()? {
        'q' => match itr.next() {
            None => Some(Commands::Quit),
            Some(_) => None,
        },
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
