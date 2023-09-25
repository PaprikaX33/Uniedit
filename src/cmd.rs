/*! Interactive parser for the input output command with the user
 */

use regex::Regex;

#[derive(Debug)]
#[allow(unused_variables)]
#[allow(dead_code)]
pub enum EncodingType {
    UTF8,
    UTF32,
    UTF32LE,
}

#[derive(Debug)]
#[allow(unused_variables)]
#[allow(dead_code)]
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
#[allow(unused_variables)]
#[allow(dead_code)]
pub enum Commands<'a> {
    Quit,
    Write {
        enc: EncodingType,
        file: Option<&'a str>,
    },
    Help,
    Compress,
    Decompress,
    AppendDec(Option<u32>),
    AppendHex(Option<u32>),
    AppendChr(Option<char>),
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

pub fn capture<'a>(inp: &'a str) -> Option<Commands<'a>> {
    match inp.trim().strip_prefix('.') {
        Some(cmd) => Some(Commands::Quit), //Command
        None => None,                      //Not a command
    }
}

//pub fn capture<'a>(inp: &'a str) -> Option<Commands<'a>> {
#[allow(dead_code)]
pub fn regex_capture<'a>(inp: &'a str) -> String {
    // "\s*(?:\.(?<cmd>\w+)(?:\|(?<arg>\w+))?(?:\|(?<attr>\w+))?$|\.0x[0-9a-fA-F]+|\.0X[0-9a-fA-F]+|\.x[0-9a-fA-F]+|\.X[0-9a-fA-F]+|\.dd|<raw>)\s*";
    //let pattern = r"\s*(?:\\.(?P<cmd>\\w+)(?:\\|(?P<arg>\\w+))?(?:\\|(?P<attr>\\w+))?$|\\.0x[0-9a-fA-F]+|\\.0X[0-9a-fA-F]+|\\.x[0-9a-fA-F]+|\\.X[0-9a-fA-F]+|\\.dd|<raw>)\\s*";
    let newer_pattern =
        r"\s*(\.(?P<cmd>\w+)\s*(?P<arg>\w*)\s*(?P<attr>\w*)|(0?x[0-9a-fA-F]+)|(\d+))\s*";
    let far_newer_pattern = r"\s*(?:\.(?P<cmd>\w+)\s*(?P<arg>\w*)\s*(?P<attr>\w*)|(?:0?[xX](?P<hex>[0-9a-fA-F]+))|(?P<dec>\d+))\s*";
    // Use r"(?x)" to enter verbose mode
    let pattern = r"\s*(.<cmd>)|(<chr>\w+)|(<digit>\d+)\s*";
    let rex = Regex::new(pattern).unwrap();
    //let captures = rex.captures(inp)?;
    let Some(captures) = rex.captures(inp) else {return "".to_string();};
    captures["cmd"].to_string()
    // match &captures["cmd"] {
    //     _ => return None,
    // }
    // return Some(Commands::Quit);
}

// Actual might working
//(?mi)^\s*(?:\.(?:(?:0?x(?P<hex>[a-f\d]+))|(?P<dec>\d+)|(?P<cmd>[\?qhecdv])))|(?P<raw>\w+)\s*$
