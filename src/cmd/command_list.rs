/*! File to declare all of the enumeration type
 */

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
/*#[allow(dead_code)]*/
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
    /// # Note
    /// Write does not perform any processing on the filename, therefore the escaping mechanism
    /// in [AppendLit](Commands::AppendLit) is bypassed.
    Write { enc: EncodingType, file: String },
    /// The help page of the program
    /// Should print all of the available command and the usage
    /// # Command
    /// + `.h`
    /// + `.?`
    Help,
    /// Compress the text in the buffer to NFC representation
    /// # Command
    /// `.c`
    Compress,
    /// Decompress the text in the buffer to NFD representation
    /// # Command
    /// `.d`
    Decompress,
    /// Append literal unicode codepoint to the buffer
    /// # Command
    /// + `.ddd`
    /// + `.0xnnn`
    /// + `.xnnn`
    ///
    /// Where `d` is decimal digit, and `n` is hexadecimal digit
    AppendLit(u32),
    /// Append the string inserted string to the buffer
    /// # Note
    /// All input that is not prefixed with `.` is considered as the raw append string
    /// # Escaping
    /// To escape the literal `.` character in the begining of the string, a `\` escape character can be used.
    /// Additionally a limited number escape characters sequence can be used.
    /// The following is all of the possible escape character
    /// + `\<space>` => `<space>` : A space character.
    /// + `\t` => `<tab>` : A tab character.
    /// + `\n` => `<newline>` : ca newline character. Interpretted as linefeed character.
    /// + `\\` => `\` : A literal backslash character.
    /// + `\.` => `.` : A literal dot character. Note: Escape is only required in front of the string.
    AppendStr(Vec<u32>),
    /// Performs literal unicode codepoint insertion to the position in the buffer.
    /// # Command
    /// + `.iddd <codepoint>`
    /// + `.innn <codepoint>`
    /// + `.ixnnn <codepoint>`
    ///
    /// Where `d` is decimal digit, and `n` is hexadecimal digit
    /// # Note
    /// Codepoint must be properly marked with a leading `.` and have the same formatting
    /// as listed in [AppendLit](Commands::AppendLit).
    ///
    /// Additional information see [AppendLit](Commands::AppendLit).
    InsertLit { pos: u32, chr: u32 },
    /// Performs string insertion in position in the buffer
    /// # Command
    /// + `.iddd <str>`
    /// + `.innn <str>`
    /// + `.ixnnn <str>`
    ///
    /// Where `d` is decimal digit, and `n` is hexadecimal digit
    /// # Note
    /// `<str>` must not contain a leading `.` in the string,
    /// otherwise it can be mistaken as [InsertLit](Commands::InsertLit).
    ///
    /// see escaping in [AppendStr](Commands::AppendStr).
    ///
    /// Additional information see [AppendLit](Commands::AppendLit).
    InsertStr { pos: u32, txt: Vec<u32> },
    /// Modify or replace a codepoint in position
    /// # Command
    /// + `.mddd <codepoint>`
    /// + `.mnnn <codepoint>`
    /// + `.mxnnn <codepoint>`
    ///
    /// As it is only capable to replace a single character, a leading `.` to denote codepoint
    /// is not required.
    Modify { pos: u32, chr: u32 },
    /// Kill or delete a character from the stream
    /// # Command
    /// + `.k ddd`
    /// + `.k 0xnnn`
    /// + `.k xnnn`
    ///
    /// Where `d` is decimal digit, and `n` is hexadecimal digit
    /// # Note
    /// All stream following the removed character is pushed forward
    Kill { pos: u32 },
    /// Print the current buffer
    ///
    /// Printing the buffer does not render the content of the buffer to the unicode,
    /// but rather prints the codepoints in the buffer.
    /// # Command
    /// `.p` Print the codepoint as decimal
    /// `.px` Print the codepoint as hexadecimal
    Print(RawBase),
    /// Purge the buffer to empty it
    /// # Command
    /// `.e`
    Erase,
    /// Render the buffer to the stdout
    /// # Note
    /// defaulted to UTF-8
    /// # Command
    /// + `.r` : Render to stdout as UTF-8
    /// + `.r32` : Render to stdout as UTF-32 Big Endian
    /// + `.r32LE` : Render to stdout as UTF-32 Little Endian
    Render(EncodingType),
    /// Validate the current buffer
    /// # Command
    /// `.v`
    Valid,
}
