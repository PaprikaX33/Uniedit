/*! Testing submodule for the cmd module

The testing for the input parsing will be located in here
 */

use super::*;

#[test]
fn parse_invalid() {
    assert_eq!(capture("."), None);
}
#[test]
fn parse_quit() {
    assert_eq!(capture(".q"), Some(Commands::Quit));
    assert_eq!(capture(".quuu"), None);
}
#[test]
fn parse_help() {
    assert_eq!(capture(".h"), Some(Commands::Help));
    assert_eq!(capture(".?"), Some(Commands::Help));
    assert_eq!(capture(".hasd"), None);
    assert_eq!(capture(".??uuu"), None);
}
#[test]
fn parse_compress() {
    assert_eq!(capture(".c"), Some(Commands::Compress));
    assert_eq!(capture(".cuuu"), None);
}
#[test]
fn parse_decompress() {
    assert_eq!(capture(".d"), Some(Commands::Decompress));
    assert_eq!(capture(".duuu"), None);
}
#[test]
fn parse_erase() {
    assert_eq!(capture(".e"), Some(Commands::Erase));
    assert_eq!(capture(".euuu"), None);
}
#[test]
fn parse_valid() {
    assert_eq!(capture(".v"), Some(Commands::Valid));
    assert_eq!(capture(".vuuu"), None);
}

#[test]
fn parse_kill() {
    assert_eq!(capture(".k 032"), Some(Commands::Kill { pos: 32 }));
    assert_eq!(capture(".k 0x2f"), Some(Commands::Kill { pos: 0x2f }));
    assert_eq!(capture(".k x7b"), Some(Commands::Kill { pos: 0x7b }));
    assert_eq!(capture(".k32"), None);
    assert_eq!(capture(".k32ff"), None);
    assert_eq!(capture(".k 0x32ba"), Some(Commands::Kill { pos: 0x32ba }));
    assert_eq!(capture(".k 20f"), None);
}
#[test]
fn parse_print() {
    assert_eq!(capture(".p"), Some(Commands::Print(RawBase::Dec)));
    assert_eq!(capture(".px"), Some(Commands::Print(RawBase::Hex)));
    assert_eq!(capture(".pu"), None);
    assert_eq!(capture(".pxaha"), None);
    assert_eq!(capture(".px no"), None);
}
#[test]
fn parse_literal_char() {
    assert_eq!(capture(".20"), Some(Commands::AppendLit(20)));
    assert_eq!(capture(".2c"), None);
    assert_eq!(capture("    .4294967297"), None);
    assert_eq!(capture("    .4294967296  "), None);
    assert_eq!(
        capture(".4294967295  "),
        Some(Commands::AppendLit(4294967295))
    );
}
#[test]
fn parse_raw() {
    assert_eq!(
        capture("230"),
        Some(Commands::AppendStr([50, 51, 48].to_vec()))
    );
    assert_eq!(
        capture("hello worlds"),
        Some(Commands::AppendStr(
            [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 115].to_vec()
        ))
    )
}

#[test]
fn parse_insertion_string() {
    assert_eq!(
        capture(".i29 hello"),
        Some(Commands::InsertStr {
            pos: 29,
            txt: [104, 101, 108, 108, 111].to_vec()
        })
    );
    assert_eq!(
        capture(".ix30 hello"),
        Some(Commands::InsertStr {
            pos: 0x30,
            txt: [104, 101, 108, 108, 111].to_vec()
        })
    );
    assert_eq!(
        capture(".i0x19 hello"),
        Some(Commands::InsertStr {
            pos: 0x19,
            txt: [104, 101, 108, 108, 111].to_vec()
        })
    );
    assert_eq!(
        capture(".i29 .2000"),
        Some(Commands::InsertLit { pos: 29, chr: 2000 })
    );
    assert_eq!(
        capture(".ix30 .10"),
        Some(Commands::InsertLit { pos: 0x30, chr: 10 })
    );
    assert_eq!(
        capture(".i0x19 .300"),
        Some(Commands::InsertLit {
            pos: 0x19,
            chr: 300,
        })
    );
    assert_eq!(capture(".i0x19.300"), None);
    assert_eq!(capture(".i0x19e300"), None);
    assert_eq!(capture(".i0x19300"), None);
}
#[test]
fn parse_render() {
    assert_eq!(capture(".r"), Some(Commands::Render(EncodingType::UTF8)));
    assert_eq!(capture(".r32"), Some(Commands::Render(EncodingType::UTF32)));
    assert_eq!(
        capture(".r32LE"),
        Some(Commands::Render(EncodingType::UTF32LE))
    );
    assert_eq!(
        capture(".r32le"),
        Some(Commands::Render(EncodingType::UTF32LE))
    );
    assert_eq!(
        capture(".r32lE"),
        Some(Commands::Render(EncodingType::UTF32LE))
    );
    assert_eq!(capture(".ra"), None);
    assert_eq!(capture(".r32uu"), None);
    assert_eq!(capture(".r a"), None);
    assert_eq!(capture(".r99"), None);
    assert_eq!(capture(".r32LEaser"), None);
    assert_eq!(capture(".r32lLea"), None);
}

#[test]
fn parse_write() {
    assert_eq!(
        capture(".w ./nyaaa"),
        Some(Commands::Write {
            enc: EncodingType::UTF8,
            file: "./nyaaa".to_string(),
        })
    );
    assert_eq!(
        capture(".w32 ./nyaaa"),
        Some(Commands::Write {
            enc: EncodingType::UTF32,
            file: "./nyaaa".to_string(),
        })
    );
    assert_eq!(
        capture(".w32lE ./nya"),
        Some(Commands::Write {
            enc: EncodingType::UTF32LE,
            file: "./nya".to_string(),
        })
    );
    assert_eq!(
        capture(".w32LE ./nyaaa"),
        Some(Commands::Write {
            enc: EncodingType::UTF32LE,
            file: "./nyaaa".to_string(),
        })
    );
    assert_eq!(capture(".w32BE ./nyaaa"), None);
    assert_eq!(capture(".w32./nyaaa"), None);
}
