/*! A collection of final validator functions to verify whether all character is consumed

This is to ensure that the function that requires less argument to not be feeded with more
argument than expected. There should be no problem on not using the validator, but it seems
to be a good practice to detect wrong command by checking the number of argument
 */

use super::command_list::Commands;

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
pub fn final_check(text: std::str::Chars, parsed: Commands) -> Option<Commands> {
    if text.as_str().is_empty() {
        Some(parsed)
    } else {
        None
    }
}

pub fn string_exact_check<'a>(
    sample: std::str::Chars<'a>,
    cmd: std::str::Chars,
) -> (bool, std::str::Chars<'a>) {
    let mut smp = sample.clone();
    for x in cmd {
        if !x.to_lowercase().eq(match smp.next() {
            None => return (false, smp),
            Some(y) => y,
        }
        .to_lowercase())
        {
            return (false, smp);
        }
    }
    (true, smp)
}
