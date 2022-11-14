use crate::additive::Additive;
use anyhow::Result;
use pest::Parser;

use super::{bytes_range::BytesRange, CRule, CommitParser};

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub id: usize,
    pub kind: TokenKind,
    pub bytes: BytesRange,
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

pub fn parse_header(header: &str) -> Result<Vec<Token>> {
    let mut v = Vec::new();
    let mut word_bytes = 0;
    let rules = CommitParser::parse(CRule::Tokens, header)?;
    let mut id = Additive::new();

    for rule in rules {
        match rule.as_rule() {
            CRule::Tokens => {
                for token in rule.into_inner() {
                    let span = token.as_span();
                    let rule = token.as_rule();

                    match rule {
                        CRule::TokenChar => {
                            let bytes = span.end() - span.start();
                            word_bytes += bytes;
                            continue;
                        }
                        _ => {
                            if word_bytes > 0 {
                                v.push(Token {
                                    id: id.stamp(),
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
                        CRule::TokenOpenBracket => {
                            v.push(Token {
                                id: id.stamp(),
                                kind: TokenKind::OpenBracket,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                            });
                        }
                        CRule::TokenCloseBracket => {
                            v.push(Token {
                                id: id.stamp(),
                                kind: TokenKind::CloseBracket,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                            });
                        }
                        CRule::TokenExclMark => {
                            v.push(Token {
                                id: id.stamp(),
                                kind: TokenKind::ExclMark,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                            });
                        }
                        CRule::TokenColon => {
                            v.push(Token {
                                id: id.stamp(),
                                kind: TokenKind::Colon,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                            });
                        }
                        CRule::TokenWhitespace => {
                            v.push(Token {
                                id: id.stamp(),
                                kind: TokenKind::Whitespace,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                            });
                        }
                        CRule::TokenEOL => {
                            v.push(Token {
                                id: id.stamp(),
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
                    id: id.stamp(),
                    kind: TokenKind::Word,
                    bytes: BytesRange {
                        start: token.bytes.end,
                        end: token.bytes.end + word_bytes,
                    },
                });
            }
            None => v.push(Token {
                id: id.stamp(),
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

#[cfg(test)]
mod parse_header {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ends_with_eol() {
        let actual = parse_header("eol\n").unwrap();
        let expected = vec![
            Token {
                id: 1,
                kind: TokenKind::Word,
                bytes: BytesRange { start: 0, end: 3 },
            },
            Token {
                id: 2,
                kind: TokenKind::EOL,
                bytes: BytesRange { start: 3, end: 4 },
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn space_at_the_start_valid_next_word_align() {
        let actual = parse_header(" space").unwrap();
        let expected = vec![
            Token {
                id: 1,
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 0, end: 1 },
            },
            Token {
                id: 2,
                kind: TokenKind::Word,
                bytes: BytesRange { start: 1, end: 6 },
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_word() {
        let actual = parse_header("fix").unwrap();
        let expected = vec![Token {
            id: 1,
            kind: TokenKind::Word,
            bytes: BytesRange { start: 0, end: 3 },
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn some_string_utf8() {
        let actual = parse_header("рад два три").unwrap();
        let expected = vec![
            Token {
                id: 1,
                kind: TokenKind::Word,
                bytes: BytesRange { start: 0, end: 6 },
            },
            Token {
                id: 2,
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 6, end: 7 },
            },
            Token {
                id: 3,
                kind: TokenKind::Word,
                bytes: BytesRange { start: 7, end: 13 },
            },
            Token {
                id: 4,
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 13, end: 14 },
            },
            Token {
                id: 5,
                kind: TokenKind::Word,
                bytes: BytesRange { start: 14, end: 20 },
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn working_commit() {
        let actual = parse_header("fix: me").unwrap();
        let expected = vec![
            Token {
                id: 1,
                kind: TokenKind::Word,
                bytes: BytesRange { start: 0, end: 3 },
            },
            Token {
                id: 2,
                kind: TokenKind::Colon,
                bytes: BytesRange { start: 3, end: 4 },
            },
            Token {
                id: 3,
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 4, end: 5 },
            },
            Token {
                id: 4,
                kind: TokenKind::Word,
                bytes: BytesRange { start: 5, end: 7 },
            },
        ];
        assert_eq!(actual, expected);
    }
}
