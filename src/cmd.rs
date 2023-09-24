/*! Interactive parser for the input output command with the user
 */

//use regex::Regex;

#[derive(Debug)]
pub enum EncodingType {
    UTF8,
    UTF32,
    UTF32LE,
}

#[derive(Debug)]
pub enum RawType {
    Dec,
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
pub enum Commands<'a> {
    Quit,
    Write {
        enc: EncodingType,
        file: Option<&'a str>,
    },
    Help,
    Compress,
    Decompress,
    AppendDec(u32),
    AppendHex(u32),
    AppendChr(char),
    Modify {
        pos: u32,
    },
    Kill {
        pos: u32,
    },
    Print(RawType),
    Erase,
    Render(EncodingType),
    Valid,
}

pub fn capture<'a>(inp: &'a str) -> Commands<'a> {
    // "\s*(?:\.(?<cmd>\w+)(?:\|(?<arg>\w+))?(?:\|(?<attr>\w+))?$|\.0x[0-9a-fA-F]+|\.0X[0-9a-fA-F]+|\.x[0-9a-fA-F]+|\.X[0-9a-fA-F]+|\.dd|<raw>)\s*";
    let pattern = r"\s*(?:\\.(?P<cmd>\\w+)(?:\\|(?P<arg>\\w+))?(?:\\|(?P<attr>\\w+))?$|\\.0x[0-9a-fA-F]+|\\.0X[0-9a-fA-F]+|\\.x[0-9a-fA-F]+|\\.X[0-9a-fA-F]+|\\.dd|<raw>)\\s*";
    //let rex = Regex::new(pattern).unwrap();
    return Commands::Quit;
}
