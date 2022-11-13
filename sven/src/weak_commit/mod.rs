use anyhow::Result;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./weak_commit/grammar.pest"] // relative to src
struct CommitParser;

#[derive(Debug, PartialEq)]
pub struct WeakCommit {
    pub rows: Vec<Row>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub bytes: BytesRange,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BytesRange {
    /// Starting byte
    pub start: usize,
    /// Ending byte
    pub end: usize,
}

impl Into<(usize, usize)> for BytesRange {
    fn into(self) -> (usize, usize) {
        (self.start, self.end)
    }
}

impl Token {
    #[inline]
    pub fn capture<'a>(&self, input: &'a str) -> &'a str {
        &input[self.bytes.start..self.bytes.end]
    }
}

impl Into<(usize, usize)> for Token {
    fn into(self) -> (usize, usize) {
        self.bytes.into()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Word,
    Whitespace,
    OpenBracket,
    CloseBracket,
    ExclMark,
    Colon,
    EOL,
}

impl WeakCommit {
    pub fn parse_header<'a>(header: &str) -> Result<Vec<Token>> {
        let mut v = Vec::new();
        let mut word_bytes = 0;
        let rules = CommitParser::parse(Rule::Tokens, header)?;

        for rule in rules {
            match rule.as_rule() {
                Rule::Tokens => {
                    for token in rule.into_inner() {
                        let span = token.as_span();
                        let rule = token.as_rule();

                        match rule {
                            Rule::TokenChar => {
                                let bytes = span.end() - span.start();
                                word_bytes += bytes;
                                continue;
                            }
                            _ => {
                                if word_bytes > 0 {
                                    v.push(Token {
                                        kind: TokenKind::Word,
                                        bytes: BytesRange {
                                            start: span.start() - word_bytes,
                                            end: span.end() - 1,
                                        },
                                    });
                                    word_bytes = 0;
                                }
                            }
                        }

                        match rule {
                            Rule::TokenOpenBracket => {
                                v.push(Token {
                                    kind: TokenKind::OpenBracket,
                                    bytes: BytesRange {
                                        start: span.start(),
                                        end: span.end(),
                                    },
                                });
                            }
                            Rule::TokenCloseBracket => {
                                v.push(Token {
                                    kind: TokenKind::CloseBracket,
                                    bytes: BytesRange {
                                        start: span.start(),
                                        end: span.end(),
                                    },
                                });
                            }
                            Rule::TokenExclMark => {
                                v.push(Token {
                                    kind: TokenKind::ExclMark,
                                    bytes: BytesRange {
                                        start: span.start(),
                                        end: span.end(),
                                    },
                                });
                            }
                            Rule::TokenColon => {
                                v.push(Token {
                                    kind: TokenKind::Colon,
                                    bytes: BytesRange {
                                        start: span.start(),
                                        end: span.end(),
                                    },
                                });
                            }
                            Rule::TokenWhitespace => {
                                v.push(Token {
                                    kind: TokenKind::Whitespace,
                                    bytes: BytesRange {
                                        start: span.start(),
                                        end: span.end(),
                                    },
                                });
                            }
                            Rule::TokenEOL => {
                                v.push(Token {
                                    kind: TokenKind::EOL,
                                    bytes: BytesRange {
                                        start: span.start(),
                                        end: span.end(),
                                    },
                                });
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        // we have to do one more iteration to clear the last word if there is any
        if word_bytes > 0 {
            match v.last() {
                Some(token) => {
                    v.push(Token {
                        kind: TokenKind::Word,
                        bytes: BytesRange {
                            start: token.bytes.end,
                            end: token.bytes.end + word_bytes,
                        },
                    });
                }
                None => v.push(Token {
                    kind: TokenKind::Word,
                    bytes: BytesRange {
                        start: 0,
                        end: word_bytes,
                    },
                }),
            }
        }

        Ok(v)
    }

    pub fn parse(commit: &str) -> Result<Self> {
        let mut rows: Vec<Row> = Vec::new();
        let mut row_n: usize = 1;

        let rules = CommitParser::parse(Rule::Lines, commit)?;

        for rule in rules {
            match rule.as_rule() {
                Rule::Lines => {
                    for rule in rule.into_inner() {
                        match rule.as_rule() {
                            Rule::Row | Rule::RowEOL => {
                                let span = rule.as_span();
                                let value = rule.as_str();
                                if !value.is_empty() {
                                    rows.push(Row {
                                        row: row_n,
                                        blank: Row::probe_blank_line(value),
                                        bytes: BytesRange {
                                            start: span.start(),
                                            end: span.end(),
                                        },
                                    });
                                    row_n += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Self { rows })
    }
}

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

    fn probe_blank_line(value: &str) -> u8 {
        match CommitParser::parse(Rule::ProbeBlankLine, value) {
            Ok(rules) => {
                for rule in rules {
                    match rule.as_rule() {
                        Rule::ProbeEOL => return 1,
                        Rule::ProbeChar => return 0,
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

#[cfg(test)]
mod parse_header {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ends_with_eol() {
        let actual = WeakCommit::parse_header("eol\n").unwrap();
        let expected = vec![
            Token {
                kind: TokenKind::Word,
                bytes: BytesRange { start: 0, end: 3 },
            },
            Token {
                kind: TokenKind::EOL,
                bytes: BytesRange { start: 3, end: 4 },
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn space_at_the_start_valid_next_word_align() {
        let actual = WeakCommit::parse_header(" space").unwrap();
        let expected = vec![
            Token {
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 0, end: 1 },
            },
            Token {
                kind: TokenKind::Word,
                bytes: BytesRange { start: 1, end: 6 },
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_word() {
        let actual = WeakCommit::parse_header("fix").unwrap();
        let expected = vec![Token {
            kind: TokenKind::Word,
            bytes: BytesRange { start: 0, end: 3 },
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn some_string_utf8() {
        let actual = WeakCommit::parse_header("рад два три").unwrap();
        let expected = vec![
            Token {
                kind: TokenKind::Word,
                bytes: BytesRange { start: 0, end: 6 },
            },
            Token {
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 6, end: 7 },
            },
            Token {
                kind: TokenKind::Word,
                bytes: BytesRange { start: 7, end: 13 },
            },
            Token {
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 13, end: 14 },
            },
            Token {
                kind: TokenKind::Word,
                bytes: BytesRange { start: 14, end: 20 },
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn working_commit() {
        let actual = WeakCommit::parse_header("fix: me").unwrap();
        let expected = vec![
            Token {
                kind: TokenKind::Word,
                bytes: BytesRange { start: 0, end: 3 },
            },
            Token {
                kind: TokenKind::Colon,
                bytes: BytesRange { start: 3, end: 4 },
            },
            Token {
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 4, end: 5 },
            },
            Token {
                kind: TokenKind::Word,
                bytes: BytesRange { start: 5, end: 7 },
            },
        ];
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod producing {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn singleline() {
        let actual = WeakCommit::parse("fix(app)!: me").unwrap();
        let expected = WeakCommit {
            rows: vec![Row {
                row: 1,
                blank: 0,
                bytes: BytesRange { start: 0, end: 13 },
            }],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiline() {
        let actual = WeakCommit::parse("one\n\ntwo\n\nthree").unwrap();
        let expected = WeakCommit {
            rows: vec![
                Row {
                    row: 1,
                    blank: 0,
                    bytes: BytesRange { start: 0, end: 4 },
                },
                Row {
                    row: 2,
                    blank: 1,
                    bytes: BytesRange { start: 4, end: 5 },
                },
                Row {
                    row: 3,
                    blank: 0,
                    bytes: BytesRange { start: 5, end: 9 },
                },
                Row {
                    row: 4,
                    blank: 1,
                    bytes: BytesRange { start: 9, end: 10 },
                },
                Row {
                    row: 5,
                    blank: 0,
                    bytes: BytesRange { start: 10, end: 15 },
                },
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiline_utf8() {
        let actual = WeakCommit::parse("раз\nдва").unwrap();
        let expected = WeakCommit {
            rows: vec![
                Row {
                    row: 1,
                    blank: 0,
                    bytes: BytesRange { start: 0, end: 7 },
                },
                Row {
                    row: 2,
                    blank: 0,
                    bytes: BytesRange { start: 7, end: 13 },
                },
            ],
        };
        assert_eq!(actual, expected);
    }
}
