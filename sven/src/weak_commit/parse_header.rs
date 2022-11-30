use super::{CRule, CommitParser};
use crate::{
    additive::Additive,
    block::{Block, Val},
    bytes::Bytes,
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
        val: Val::Root,
        domain: Some(Domain::Root),
    }];

    let rules = CommitParser::parse(CRule::Tokens, header)?;
    let mut prev = 0;

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
                            prev = span.end();
                            if word_bytes > 0 {
                                v.push(Block {
                                    id: id.stamp(),
                                    found_at: found_at.stamp(),
                                    val: Val::Seq(Bytes::new(
                                        span.start() - word_bytes,
                                        span.end() - 1,
                                    )),
                                    domain: None,
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
                                domain: None,
                            });
                        }
                        CRule::TokenCloseBracket => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                val: Val::CloseBracket,
                                domain: None,
                            });
                        }
                        CRule::TokenExclMark => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                val: Val::ExclMark,
                                domain: None,
                            });
                        }
                        CRule::TokenColon => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                val: Val::Colon,
                                domain: None,
                            });
                        }
                        CRule::TokenWhitespace => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                val: Val::Space,
                                domain: None,
                            });
                        }
                        CRule::TokenEOL => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                val: Val::EOL,
                                domain: None,
                            });
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    if word_bytes > 0 {
        v.push(Block {
            id: id.stamp(),
            found_at: found_at.stamp(),
            val: Val::Seq(Bytes::new(prev, prev + word_bytes)),
            domain: None,
        });
    }

    Ok(BTreeSet::from_iter(v.into_iter()))
}

#[cfg(test)]
mod rows {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let source = String::from("");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![Block {
            id: 0,
            found_at: 0,
            val: Val::Root,
            domain: Some(Domain::Root),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn eol() {
        let source = String::from("\n");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![
            Block {
                id: 0,
                found_at: 0,
                val: Val::Root,
                domain: Some(Domain::Root),
            },
            Block {
                id: 1024,
                found_at: 1,
                val: Val::EOL,
                domain: None,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_word() {
        let source = String::from("one");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![
            Block {
                id: 0,
                found_at: 0,
                val: Val::Root,
                domain: Some(Domain::Root),
            },
            Block {
                id: 1024,
                found_at: 1,
                val: Val::Seq(Bytes::new(0, 3)),
                domain: None,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_word_and_eol() {
        let source = String::from("one\n");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![
            Block {
                id: 0,
                found_at: 0,
                val: Val::Root,
                domain: Some(Domain::Root),
            },
            Block {
                id: 1024,
                found_at: 1,
                val: Val::Seq(Bytes::new(0, 3)),
                domain: None,
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                val: Val::EOL,
                domain: None,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn text() {
        let source = String::from("just some text");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![
            Block {
                id: 0,
                found_at: 0,
                val: Val::Root,
                domain: Some(Domain::Root),
            },
            Block {
                id: 1024,
                found_at: 1,
                val: Val::Seq(Bytes::new(0, 4)),
                domain: None,
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                val: Val::Space,
                domain: None,
            },
            Block {
                id: 1024 * 3,
                found_at: 3,
                val: Val::Seq(Bytes::new(5, 9)),
                domain: None,
            },
            Block {
                id: 1024 * 4,
                found_at: 4,
                val: Val::Space,
                domain: None,
            },
            Block {
                id: 1024 * 5,
                found_at: 5,
                val: Val::Seq(Bytes::new(10, 14)),
                domain: None,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn commit() {
        let source = String::from("fix(app)!: me");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![
            Block {
                id: 0,
                found_at: 0,
                val: Val::Root,
                domain: Some(Domain::Root),
            },
            Block {
                id: 1024,
                found_at: 1,
                val: Val::Seq(Bytes::new(0, 3)),
                domain: None,
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                val: Val::OpenBracket,
                domain: None,
            },
            Block {
                id: 1024 * 3,
                found_at: 3,
                val: Val::Seq(Bytes::new(4, 7)),
                domain: None,
            },
            Block {
                id: 1024 * 4,
                found_at: 4,
                val: Val::CloseBracket,
                domain: None,
            },
            Block {
                id: 1024 * 5,
                found_at: 5,
                val: Val::ExclMark,
                domain: None,
            },
            Block {
                id: 1024 * 6,
                found_at: 6,
                val: Val::Colon,
                domain: None,
            },
            Block {
                id: 1024 * 7,
                found_at: 7,
                val: Val::Space,
                domain: None,
            },
            Block {
                id: 1024 * 8,
                found_at: 8,
                val: Val::Seq(Bytes::new(11, 13)),
                domain: None,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn utf8() {
        let source = String::from("fix: да");
        let actual = Vec::from_iter(parse_header(&source).unwrap());
        let expected = vec![
            Block {
                id: 0,
                found_at: 0,
                val: Val::Root,
                domain: Some(Domain::Root),
            },
            Block {
                id: 1024,
                found_at: 1,
                val: Val::Seq(Bytes::new(0, 3)),
                domain: None,
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                val: Val::Colon,
                domain: None,
            },
            Block {
                id: 1024 * 3,
                found_at: 3,
                val: Val::Space,
                domain: None,
            },
            Block {
                id: 1024 * 4,
                found_at: 4,
                val: Val::Seq(Bytes::new(5, 9)),
                domain: None,
            },
        ];
        assert_eq!(actual, expected);
    }
}
