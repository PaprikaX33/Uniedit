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
