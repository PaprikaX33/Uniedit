/*! Interactive parser for the input output command with the user
 */

//use regex::Regex;

#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
#[allow(unused_variables)]
#[allow(dead_code)]
pub enum Commands {
    /// Quit the program
    /// # Command
    /// + `.q`
    Quit,
    /// Write the buffer to a file
    /// # Command
    /// + `.w` : Write as UTF-8.
    /// + `.w32` : Write as UTF-32 Big Endian.
    /// + `.w32LE` : Write as UTF-32 Little Endian.
    /// + `.w <file>` : Write as UTF-8 to *file*.
    /// + `.w32 <file>` : Write as UTF-32 Big Endian to *file*.
    /// + `.w32LE <file>` : Write as UTF-32 Little Endian to *file*.
    Write {
        enc: EncodingType,
        file: Option<String>,
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
    AppendLit(Option<u32>),
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
        pos: Option<u32>,
        chr: Option<u32>,
    },
    /// Kill or delete a character from the stream
    /// # Note
    /// All stream following the removed character is pushed forward
    Kill {
        pos: Option<u32>,
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

fn parse_cmd_selection(inp: std::str::Chars) -> Option<Commands> {
    let mut itr = inp;
    match itr.next().unwrap() {
        'q' => Some(Commands::Quit),
        _ => Some(Commands::Quit),
    }
}

fn parse_raw(inp: &str) -> Option<Commands> {
    let mut bff: Vec<u32> = Vec::new();
    let mut iters = inp.chars();
    while let Some(chr) = iters.next() {
        match chr {
            '\\' => {
                let Some(nxt) = iters.next() else { return None };
                match nxt {
                    //' ' => bff.push(0x20),
                    ' ' => bff.push(' ' as u32),
                    'n' => bff.push('\n' as u32),
                    't' => bff.push('\t' as u32),
                    '\\' => bff.push('\\' as u32),
                    _ => return None,
                }
            }
            _ => bff.push(chr as u32), // No escape
        }
    }
    Some(Commands::AppendStr(bff))
}
