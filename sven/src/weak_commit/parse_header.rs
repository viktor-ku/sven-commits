use super::{bytes_range::BytesRange, CRule, CommitParser};
use crate::{
    additive::Additive,
    block::{Block, Info, Val},
    domain::Domain,
};
use anyhow::Result;
use pest::Parser;
use std::collections::BTreeSet;

pub fn parse_header(header: &str) -> Result<BTreeSet<Block>> {
    let mut word_bytes = 0;
    let mut found_at = Additive::new();
    let mut id = Additive { step: 1024, val: 0 };
    let mut v = vec![Block {
        id: id.stamp(),
        found_at: found_at.stamp(),
        bytes: BytesRange { start: 0, end: 0 },
        val: Val::Root,
        info: Info {
            domain: Some(Domain::Root),
        },
    }];

    let rules = CommitParser::parse(CRule::Tokens, header)?;

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
                                v.push(Block {
                                    id: id.stamp(),
                                    found_at: found_at.stamp(),
                                    val: Val::Seq,
                                    bytes: BytesRange {
                                        start: span.start() - word_bytes,
                                        end: span.end() - 1,
                                    },
                                    info: Info::default(),
                                });
                                word_bytes = 0;
                            }
                        }
                    }

                    match rule {
                        CRule::TokenOpenBracket => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                val: Val::OpenBracket,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
                            });
                        }
                        CRule::TokenCloseBracket => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                val: Val::CloseBracket,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
                            });
                        }
                        CRule::TokenExclMark => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                val: Val::ExclMark,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
                            });
                        }
                        CRule::TokenColon => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                val: Val::Colon,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
                            });
                        }
                        CRule::TokenWhitespace => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                val: Val::Space,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
                            });
                        }
                        CRule::TokenEOL => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                val: Val::EOL,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
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
                v.push(Block {
                    id: id.stamp(),
                    found_at: found_at.stamp(),
                    val: Val::Seq,
                    bytes: BytesRange {
                        start: token.bytes.end,
                        end: token.bytes.end + word_bytes,
                    },
                    info: Info::default(),
                });
            }
            None => v.push(Block {
                id: id.stamp(),
                found_at: found_at.stamp(),
                val: Val::Seq,
                bytes: BytesRange {
                    start: 0,
                    end: word_bytes,
                },
                info: Info::default(),
            }),
        }
    }

    Ok(BTreeSet::from_iter(v.into_iter()))
}

#[cfg(test)]
mod rows {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ends_with_eol() {
        let source = String::from("eol\n");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![
            Block {
                id: 0,
                found_at: 0,
                val: Val::Root,
                bytes: BytesRange::empty(0),
                info: Info {
                    domain: Some(Domain::Root),
                },
            },
            Block {
                id: 1024,
                found_at: 1,
                val: Val::Seq,
                bytes: BytesRange { start: 0, end: 3 },
                info: Info::default(),
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                val: Val::EOL,
                bytes: BytesRange { start: 3, end: 4 },
                info: Info::default(),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn space_at_the_start_valid_next_word_align() {
        let source = String::from(" space");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![
            Block {
                id: 0,
                found_at: 0,
                val: Val::Root,
                bytes: BytesRange::empty(0),
                info: Info {
                    domain: Some(Domain::Root),
                },
            },
            Block {
                id: 1024,
                found_at: 1,
                val: Val::Space,
                bytes: BytesRange { start: 0, end: 1 },
                info: Info::default(),
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                val: Val::Seq,
                bytes: BytesRange { start: 1, end: 6 },
                info: Info::default(),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_word() {
        let source = String::from("fix");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![
            Block {
                id: 0,
                found_at: 0,
                val: Val::Root,
                bytes: BytesRange::empty(0),
                info: Info {
                    domain: Some(Domain::Root),
                },
            },
            Block {
                id: 1024,
                found_at: 1,
                val: Val::Seq,
                bytes: BytesRange { start: 0, end: 3 },
                info: Info::default(),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn some_string_utf8() {
        let source = String::from("рад два три");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![
            Block {
                id: 0,
                found_at: 0,
                val: Val::Root,
                bytes: BytesRange::empty(0),
                info: Info {
                    domain: Some(Domain::Root),
                },
            },
            Block {
                id: 1024,
                found_at: 1,
                val: Val::Seq,
                bytes: BytesRange { start: 0, end: 6 },
                info: Info::default(),
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                val: Val::Space,
                bytes: BytesRange { start: 6, end: 7 },
                info: Info::default(),
            },
            Block {
                id: 1024 * 3,
                found_at: 3,
                val: Val::Seq,
                bytes: BytesRange { start: 7, end: 13 },
                info: Info::default(),
            },
            Block {
                id: 1024 * 4,
                found_at: 4,
                val: Val::Space,
                bytes: BytesRange { start: 13, end: 14 },
                info: Info::default(),
            },
            Block {
                id: 1024 * 5,
                found_at: 5,
                val: Val::Seq,
                bytes: BytesRange { start: 14, end: 20 },
                info: Info::default(),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn working_commit() {
        let source = String::from("fix: me");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![
            Block {
                id: 0,
                found_at: 0,
                val: Val::Root,
                bytes: BytesRange::empty(0),
                info: Info {
                    domain: Some(Domain::Root),
                },
            },
            Block {
                id: 1024,
                found_at: 1,
                val: Val::Seq,
                bytes: BytesRange { start: 0, end: 3 },
                info: Info::default(),
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                val: Val::Colon,
                bytes: BytesRange { start: 3, end: 4 },
                info: Info::default(),
            },
            Block {
                id: 1024 * 3,
                found_at: 3,
                val: Val::Space,
                bytes: BytesRange { start: 4, end: 5 },
                info: Info::default(),
            },
            Block {
                id: 1024 * 4,
                found_at: 4,
                val: Val::Seq,
                bytes: BytesRange { start: 5, end: 7 },
                info: Info::default(),
            },
        ];
        assert_eq!(actual, expected);
    }
}
