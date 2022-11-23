use super::{bytes_range::BytesRange, CRule, CommitParser};
use crate::additive::Additive;
use anyhow::Result;
use pest::Parser;
use std::fmt::Debug;

#[derive(PartialEq, Eq)]
pub struct Token {
    pub id: usize,
    pub kind: TokenKind,
    pub bytes: BytesRange,

    #[cfg(debug_assertions)]
    pub source: String,
}

impl Token {
    #[inline]
    pub fn capture<'a>(&self, input: &'a str) -> &'a str {
        &input[self.bytes.start..self.bytes.end]
    }

    /// Total bytes which token takes
    #[inline]
    pub fn total(&self) -> usize {
        self.bytes.total()
    }
}

impl Into<(usize, usize)> for Token {
    fn into(self) -> (usize, usize) {
        self.bytes.into()
    }
}

#[cfg(debug_assertions)]
impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = {
            let kind_str = self.kind.stringify();
            let len = kind_str.len();
            let diff = 10 - len;
            format!("{:?}{}", self.kind, " ".repeat(diff))
        };

        write!(
            f,
            "{} {} {:?} \"{}\"",
            self.id,
            kind,
            self.bytes,
            match self.kind {
                TokenKind::EOL => {
                    "\\n"
                }
                _ => self.capture(&self.source),
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// Any sequence of any utf8 characters, excluding other kinds of token
    Seq,
    Whitespace,
    OpenBracket,
    CloseBracket,
    ExclMark,
    Colon,
    EOL,
}

impl TokenKind {
    pub fn stringify<'a>(&self) -> &'a str {
        match self {
            TokenKind::Seq => "Seq",
            TokenKind::Whitespace => "Whitespace",
            TokenKind::OpenBracket => "OpenBracket",
            TokenKind::CloseBracket => "CloseBracket",
            TokenKind::ExclMark => "ExclMark",
            TokenKind::Colon => "Colon",
            TokenKind::EOL => "EOL",
        }
    }
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
                                    kind: TokenKind::Seq,
                                    bytes: BytesRange {
                                        start: span.start() - word_bytes,
                                        end: span.end() - 1,
                                    },
                                    #[cfg(debug_assertions)]
                                    source: header.to_string(),
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
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
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
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
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
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
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
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
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
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
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
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
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
                    kind: TokenKind::Seq,
                    bytes: BytesRange {
                        start: token.bytes.end,
                        end: token.bytes.end + word_bytes,
                    },
                    #[cfg(debug_assertions)]
                    source: header.to_string(),
                });
            }
            None => v.push(Token {
                id: id.stamp(),
                kind: TokenKind::Seq,
                bytes: BytesRange {
                    start: 0,
                    end: word_bytes,
                },
                #[cfg(debug_assertions)]
                source: header.to_string(),
            }),
        }
    }

    Ok(v)
}

#[cfg(test)]
mod rows {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ends_with_eol() {
        let source = String::from("eol\n");
        let actual = parse_header(&source).unwrap();
        let expected = vec![
            Token {
                id: 0,
                kind: TokenKind::Seq,
                bytes: BytesRange { start: 0, end: 3 },
                source: source.clone(),
            },
            Token {
                id: 1,
                kind: TokenKind::EOL,
                bytes: BytesRange { start: 3, end: 4 },
                source: source.clone(),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn space_at_the_start_valid_next_word_align() {
        let source = String::from(" space");
        let actual = parse_header(&source).unwrap();
        let expected = vec![
            Token {
                id: 0,
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 0, end: 1 },
                source: source.clone(),
            },
            Token {
                id: 1,
                kind: TokenKind::Seq,
                bytes: BytesRange { start: 1, end: 6 },
                source: source.clone(),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_word() {
        let source = String::from("fix");
        let actual = parse_header(&source).unwrap();
        let expected = vec![Token {
            id: 0,
            kind: TokenKind::Seq,
            bytes: BytesRange { start: 0, end: 3 },
            source: source.clone(),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn some_string_utf8() {
        let source = String::from("рад два три");
        let actual = parse_header(&source).unwrap();
        let expected = vec![
            Token {
                id: 0,
                kind: TokenKind::Seq,
                bytes: BytesRange { start: 0, end: 6 },
                source: source.clone(),
            },
            Token {
                id: 1,
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 6, end: 7 },
                source: source.clone(),
            },
            Token {
                id: 2,
                kind: TokenKind::Seq,
                bytes: BytesRange { start: 7, end: 13 },
                source: source.clone(),
            },
            Token {
                id: 3,
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 13, end: 14 },
                source: source.clone(),
            },
            Token {
                id: 4,
                kind: TokenKind::Seq,
                bytes: BytesRange { start: 14, end: 20 },
                source: source.clone(),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn working_commit() {
        let source = String::from("fix: me");
        let actual = parse_header(&source).unwrap();
        let expected = vec![
            Token {
                id: 0,
                kind: TokenKind::Seq,
                bytes: BytesRange { start: 0, end: 3 },
                source: source.clone(),
            },
            Token {
                id: 1,
                kind: TokenKind::Colon,
                bytes: BytesRange { start: 3, end: 4 },
                source: source.clone(),
            },
            Token {
                id: 2,
                kind: TokenKind::Whitespace,
                bytes: BytesRange { start: 4, end: 5 },
                source: source.clone(),
            },
            Token {
                id: 3,
                kind: TokenKind::Seq,
                bytes: BytesRange { start: 5, end: 7 },
                source: source.clone(),
            },
        ];
        assert_eq!(actual, expected);
    }
}
