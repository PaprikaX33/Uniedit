/*! Testing submodule for the cmd module

The testing for the input parsing will be located in here
 */

use super::capture;
use super::Commands;
#[test]
fn test_invalid() {
    assert_eq!(capture("."), None);
}
#[test]
fn test_quit() {
    assert_eq!(capture(".q"), Some(Commands::Quit));
    assert_eq!(capture(".quuu"), None);
}
#[test]
fn test_literal_char() {
    assert_eq!(capture(".20"), Some(Commands::AppendLit(20)));
    assert_eq!(capture("    .4294967297"), None);
    assert_eq!(capture("    .4294967296  "), None);
    assert_eq!(
        capture(".4294967295  "),
        Some(Commands::AppendLit(4294967295))
    );
}
