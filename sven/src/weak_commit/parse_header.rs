use super::{CRule, CommitParser};
use crate::{
    additive::Additive,
    block::{Block, Status, Val},
    bytes::Bytes,
    domain::Domain,
};
use anyhow::Result;
use pest::Parser;

pub fn parse_header(header: &str) -> Result<Vec<Block>> {
    let mut word_bytes = 0;
    let mut id = Additive {
        step: 1_000,
        val: 0,
    };
    let mut v = vec![Block {
        id: Some(id.stamp()),
        val: Val::Root,
        domain: Domain::Root,
        bytes: None,
        status: Status::Settled,
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
                                    id: Some(id.stamp()),
                                    val: Val::Seq,
                                    bytes: Some(Bytes::new(
                                        span.start() - word_bytes,
                                        span.end() - 1,
                                    )),
                                    domain: Domain::None,
                                    status: Status::Unsigned,
                                });
                                word_bytes = 0;
                            }
                        }
                    }

                    match rule {
                        CRule::TokenOpenBracket => {
                            v.push(Block {
                                id: Some(id.stamp()),
                                val: Val::OpenBracket,
                                domain: Domain::None,
                                bytes: Some(span.into()),
                                status: Status::Unsigned,
                            });
                        }
                        CRule::TokenCloseBracket => {
                            v.push(Block {
                                id: Some(id.stamp()),
                                val: Val::CloseBracket,
                                domain: Domain::None,
                                bytes: Some(span.into()),
                                status: Status::Unsigned,
                            });
                        }
                        CRule::TokenExclMark => {
                            v.push(Block {
                                id: Some(id.stamp()),
                                val: Val::ExclMark,
                                domain: Domain::None,
                                bytes: Some(span.into()),
                                status: Status::Unsigned,
                            });
                        }
                        CRule::TokenColon => {
                            v.push(Block {
                                id: Some(id.stamp()),
                                val: Val::Colon,
                                domain: Domain::None,
                                bytes: Some(span.into()),
                                status: Status::Unsigned,
                            });
                        }
                        CRule::TokenWhitespace => {
                            v.push(Block {
                                id: Some(id.stamp()),
                                val: Val::Space,
                                domain: Domain::None,
                                bytes: Some(span.into()),
                                status: Status::Unsigned,
                            });
                        }
                        CRule::TokenEOL => {
                            v.push(Block {
                                id: Some(id.stamp()),
                                val: Val::EOL,
                                domain: Domain::None,
                                bytes: Some(span.into()),
                                status: Status::Unsigned,
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
            id: Some(id.stamp()),
            val: Val::Seq,
            domain: Domain::None,
            bytes: Some(Bytes::new(prev, prev + word_bytes)),
            status: Status::Unsigned,
        });
    }

    Ok(v)
}

#[cfg(test)]
mod rows {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let source = String::from("");
        let actual = parse_header(&source).unwrap();
        let expected = vec![Block::root()];
        assert_eq!(actual, expected);
    }

    #[test]
    fn eol() {
        let source = String::from("\n");
        let actual = parse_header(&source).unwrap();
        let expected = vec![
            Block::root(),
            Block {
                id: Some(1_000),
                val: Val::EOL,
                domain: Domain::None,
                bytes: Some(Bytes::new(0, 1)),
                status: Status::Unsigned,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_word() {
        let source = String::from("one");
        let actual = parse_header(&source).unwrap();
        let expected = vec![
            Block::root(),
            Block {
                id: Some(1_000),
                val: Val::Seq,
                domain: Domain::None,
                bytes: Some(Bytes::new(0, 3)),
                status: Status::Unsigned,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_word_and_eol() {
        let source = String::from("one\n");
        let actual = parse_header(&source).unwrap();
        let expected = vec![
            Block::root(),
            Block {
                id: Some(1_000),
                val: Val::Seq,
                domain: Domain::None,
                bytes: Some(Bytes::new(0, 3)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(2_000),
                val: Val::EOL,
                domain: Domain::None,
                bytes: Some(Bytes::new(3, 4)),
                status: Status::Unsigned,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn text() {
        let source = String::from("just some text");
        let actual = parse_header(&source).unwrap();
        let expected = vec![
            Block::root(),
            Block {
                id: Some(1_000),
                val: Val::Seq,
                domain: Domain::None,
                bytes: Some(Bytes::new(0, 4)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(2_000),
                val: Val::Space,
                domain: Domain::None,
                bytes: Some(Bytes::new(4, 5)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(3_000),
                val: Val::Seq,
                domain: Domain::None,
                bytes: Some(Bytes::new(5, 9)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(4_000),
                val: Val::Space,
                domain: Domain::None,
                bytes: Some(Bytes::new(9, 10)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(5_000),
                val: Val::Seq,
                domain: Domain::None,
                bytes: Some(Bytes::new(10, 14)),
                status: Status::Unsigned,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn commit() {
        let source = String::from("fix(app)!: me");
        let actual = parse_header(&source).unwrap();
        let expected = vec![
            Block::root(),
            Block {
                id: Some(1_000),
                val: Val::Seq,
                domain: Domain::None,
                bytes: Some(Bytes::new(0, 3)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(2_000),
                val: Val::OpenBracket,
                domain: Domain::None,
                bytes: Some(Bytes::new(3, 4)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(3_000),
                val: Val::Seq,
                domain: Domain::None,
                bytes: Some(Bytes::new(4, 7)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(4_000),
                val: Val::CloseBracket,
                domain: Domain::None,
                bytes: Some(Bytes::new(7, 8)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(5_000),
                val: Val::ExclMark,
                domain: Domain::None,
                bytes: Some(Bytes::new(8, 9)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(6_000),
                val: Val::Colon,
                domain: Domain::None,
                bytes: Some(Bytes::new(9, 10)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(7_000),
                val: Val::Space,
                domain: Domain::None,
                bytes: Some(Bytes::new(10, 11)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(8_000),
                val: Val::Seq,
                domain: Domain::None,
                bytes: Some(Bytes::new(11, 13)),
                status: Status::Unsigned,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn utf8() {
        let source = String::from("fix: да");
        let actual = parse_header(&source).unwrap();
        let expected = vec![
            Block::root(),
            Block {
                id: Some(1_000),
                val: Val::Seq,
                domain: Domain::None,
                bytes: Some(Bytes::new(0, 3)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(2_000),
                val: Val::Colon,
                domain: Domain::None,
                bytes: Some(Bytes::new(3, 4)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(3_000),
                val: Val::Space,
                domain: Domain::None,
                bytes: Some(Bytes::new(4, 5)),
                status: Status::Unsigned,
            },
            Block {
                id: Some(4_000),
                val: Val::Seq,
                domain: Domain::None,
                bytes: Some(Bytes::new(5, 9)),
                status: Status::Unsigned,
            },
        ];
        assert_eq!(actual, expected);
    }
}
