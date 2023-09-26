/*! Testing submodule for the cmd module

The testing for the input parsing will be located in here
 */

use super::capture;
use super::Commands;
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
fn parse_literal_char() {
    assert_eq!(capture(".20"), Some(Commands::AppendLit(20)));
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
