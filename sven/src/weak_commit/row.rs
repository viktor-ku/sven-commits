use super::{
    bytes_range::BytesRange,
    parser::{CRule, CommitParser},
};
use pest::Parser;

/// Metadata about each row.
#[derive(Debug, PartialEq)]
pub struct Row {
    /// Consists of two integers indicating the start byte index
    /// of the row and the end byte index of the row from the start of the
    /// input.
    pub bytes: BytesRange,

    /// The row starting 1.
    pub row: usize,

    /// 1 for the new line,
    /// 0 for any other character
    pub blank: u8,
}

impl Row {
    /// Get the expected index for the current row, if being captured from an array of relevant lines:
    /// `lines[row.row_index()]` in a safe way.
    #[inline]
    pub fn row_index(&self) -> usize {
        self.row.checked_sub(1).unwrap_or(0)
    }

    pub fn probe_blank_line(value: &str) -> u8 {
        match CommitParser::parse(CRule::ProbeBlankLine, value) {
            Ok(rules) => {
                for rule in rules {
                    match rule.as_rule() {
                        CRule::ProbeEOL => return 1,
                        CRule::ProbeChar => return 0,
                        _ => unreachable!(),
                    };
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        };

        unreachable!()
    }
}
