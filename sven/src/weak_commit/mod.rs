use anyhow::Result;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./weak_commit/grammar.pest"] // relative to src
struct CommitParser;

#[derive(Debug, PartialEq)]
pub struct WeakCommit<'a> {
    pub rows: Vec<Row<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    // start byte
    pub start: usize,
    // end byte
    pub end: usize,
}

impl Token {
    #[inline]
    pub fn capture<'a>(&self, input: &'a str) -> &'a str {
        &input[self.start..self.end]
    }
}

impl Into<(usize, usize)> for Token {
    fn into(self) -> (usize, usize) {
        (self.start, self.end)
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

impl<'a> WeakCommit<'a> {
    pub fn parse_header(&self) -> Result<Vec<Token>> {
        let mut v = Vec::new();

        if self.rows.is_empty() {
            return Ok(v);
        }

        let input = self.rows[0].value;
        let rules = CommitParser::parse(Rule::Tokens, input)?;
        let mut word_bytes = 0;

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
                                        start: span.start() - word_bytes,
                                        end: span.start(),
                                    });
                                    word_bytes = 0;
                                }
                            }
                        }

                        match rule {
                            Rule::TokenOpenBracket => {
                                v.push(Token {
                                    kind: TokenKind::OpenBracket,
                                    start: span.start(),
                                    end: span.end(),
                                });
                            }
                            Rule::TokenCloseBracket => {
                                v.push(Token {
                                    kind: TokenKind::CloseBracket,
                                    start: span.start(),
                                    end: span.end(),
                                });
                            }
                            Rule::TokenExclMark => {
                                v.push(Token {
                                    kind: TokenKind::ExclMark,
                                    start: span.start(),
                                    end: span.end(),
                                });
                            }
                            Rule::TokenColon => {
                                v.push(Token {
                                    kind: TokenKind::Colon,
                                    start: span.start(),
                                    end: span.end(),
                                });
                            }
                            Rule::TokenWhitespace => {
                                v.push(Token {
                                    kind: TokenKind::Whitespace,
                                    start: span.start(),
                                    end: span.end(),
                                });
                            }
                            Rule::TokenEOL => {
                                v.push(Token {
                                    kind: TokenKind::EOL,
                                    start: span.start(),
                                    end: span.end(),
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
                        start: token.end,
                        end: token.end + word_bytes,
                    });
                }
                None => v.push(Token {
                    kind: TokenKind::Word,
                    start: 0,
                    end: word_bytes,
                }),
            }
        }

        Ok(v)
    }

    pub fn parse(commit: &'a str) -> Result<Self> {
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
                                        value,
                                        row: row_n,
                                        range_bytes: (span.start(), span.end()),
                                        blank: Row::probe_blank_line(value),
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

#[derive(Debug, PartialEq)]
pub struct Row<'row> {
    /// Consists of two integers indicating the start byte index
    /// of the row and the end byte index of the row from the start of the
    /// input.
    pub range_bytes: (usize, usize),

    /// The row starting 1.
    pub row: usize,

    /// An actual row str
    pub value: &'row str,

    /// 1 for the new line,
    /// 0 for any other character
    pub blank: u8,
}

impl<'row> Row<'row> {
    /// Get the expected index for the current row, if being captured from an array of relevant lines:
    /// `lines[row.row_index()]` in a safe way.
    #[inline]
    pub fn row_index(&self) -> usize {
        self.row.checked_sub(1).unwrap_or(0)
    }

    fn probe_blank_line(value: &'row str) -> u8 {
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
        let commit = WeakCommit::parse("eol\n").unwrap();
        let actual = commit.parse_header().unwrap();
        let expected = vec![
            Token {
                kind: TokenKind::Word,
                start: 0,
                end: 3,
            },
            Token {
                kind: TokenKind::EOL,
                start: 3,
                end: 4,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn space_at_the_start_valid_next_word_align() {
        let commit = WeakCommit::parse(" space").unwrap();
        let actual = commit.parse_header().unwrap();
        let expected = vec![
            Token {
                kind: TokenKind::Whitespace,
                start: 0,
                end: 1,
            },
            Token {
                kind: TokenKind::Word,
                start: 1,
                end: 6,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_word() {
        let commit = WeakCommit::parse("fix").unwrap();
        let actual = commit.parse_header().unwrap();
        let expected = vec![Token {
            kind: TokenKind::Word,
            start: 0,
            end: 3,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn some_string_utf8() {
        let commit = WeakCommit::parse("рад два три").unwrap();
        let actual = commit.parse_header().unwrap();
        let expected = vec![
            Token {
                kind: TokenKind::Word,
                start: 0,
                end: 6,
            },
            Token {
                kind: TokenKind::Whitespace,
                start: 6,
                end: 7,
            },
            Token {
                kind: TokenKind::Word,
                start: 7,
                end: 13,
            },
            Token {
                kind: TokenKind::Whitespace,
                start: 13,
                end: 14,
            },
            Token {
                kind: TokenKind::Word,
                start: 14,
                end: 20,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn working_commit() {
        let commit = WeakCommit::parse("fix: me").unwrap();
        let actual = commit.parse_header().unwrap();
        let expected = vec![
            Token {
                kind: TokenKind::Word,
                start: 0,
                end: 3,
            },
            Token {
                kind: TokenKind::Colon,
                start: 3,
                end: 4,
            },
            Token {
                kind: TokenKind::Whitespace,
                start: 4,
                end: 5,
            },
            Token {
                kind: TokenKind::Word,
                start: 5,
                end: 7,
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
                range_bytes: (0, 13),
                row: 1,
                value: "fix(app)!: me",
                blank: 0,
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
                    range_bytes: (0, 4),
                    row: 1,
                    value: "one\n",
                    blank: 0,
                },
                Row {
                    range_bytes: (4, 5),
                    row: 2,
                    value: "\n",
                    blank: 1,
                },
                Row {
                    range_bytes: (5, 9),
                    row: 3,
                    value: "two\n",
                    blank: 0,
                },
                Row {
                    range_bytes: (9, 10),
                    row: 4,
                    value: "\n",
                    blank: 1,
                },
                Row {
                    range_bytes: (10, 15),
                    row: 5,
                    value: "three",
                    blank: 0,
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
                    range_bytes: (0, 7),
                    row: 1,
                    value: "раз\n",
                    blank: 0,
                },
                Row {
                    range_bytes: (7, 13),
                    row: 2,
                    value: "два",
                    blank: 0,
                },
            ],
        };
        assert_eq!(actual, expected);
    }
}
