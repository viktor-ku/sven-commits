use super::{bytes_range::BytesRange, CRule, CommitParser};
use crate::{
    additive::Additive,
    block::{Block, Info, Kind},
    subject::Subject,
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
        kind: Kind::Root,
        info: Info {
            subject: Some(Subject::Root),
        },
        #[cfg(debug_assertions)]
        source: header.to_string(),
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
                                    kind: Kind::Seq,
                                    bytes: BytesRange {
                                        start: span.start() - word_bytes,
                                        end: span.end() - 1,
                                    },
                                    info: Info::default(),
                                    #[cfg(debug_assertions)]
                                    source: header.to_string(),
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
                                kind: Kind::OpenBracket,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
                            });
                        }
                        CRule::TokenCloseBracket => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                kind: Kind::CloseBracket,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
                            });
                        }
                        CRule::TokenExclMark => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                kind: Kind::ExclMark,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
                            });
                        }
                        CRule::TokenColon => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                kind: Kind::Colon,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
                            });
                        }
                        CRule::TokenWhitespace => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                kind: Kind::Space,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
                            });
                        }
                        CRule::TokenEOL => {
                            v.push(Block {
                                id: id.stamp(),
                                found_at: found_at.stamp(),
                                kind: Kind::EOL,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                info: Info::default(),
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
                v.push(Block {
                    id: id.stamp(),
                    found_at: found_at.stamp(),
                    kind: Kind::Seq,
                    bytes: BytesRange {
                        start: token.bytes.end,
                        end: token.bytes.end + word_bytes,
                    },
                    info: Info::default(),
                    #[cfg(debug_assertions)]
                    source: header.to_string(),
                });
            }
            None => v.push(Block {
                id: id.stamp(),
                found_at: found_at.stamp(),
                kind: Kind::Seq,
                bytes: BytesRange {
                    start: 0,
                    end: word_bytes,
                },
                info: Info::default(),
                #[cfg(debug_assertions)]
                source: header.to_string(),
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
                kind: Kind::Root,
                bytes: BytesRange::empty(0),
                info: Info {
                    subject: Some(Subject::Root),
                },
                source: source.clone(),
            },
            Block {
                id: 1024,
                found_at: 1,
                kind: Kind::Seq,
                bytes: BytesRange { start: 0, end: 3 },
                info: Info::default(),
                source: source.clone(),
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                kind: Kind::EOL,
                bytes: BytesRange { start: 3, end: 4 },
                info: Info::default(),
                source: source.clone(),
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
                kind: Kind::Root,
                bytes: BytesRange::empty(0),
                info: Info {
                    subject: Some(Subject::Root),
                },
                source: source.clone(),
            },
            Block {
                id: 1024,
                found_at: 1,
                kind: Kind::Space,
                bytes: BytesRange { start: 0, end: 1 },
                info: Info::default(),
                source: source.clone(),
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                kind: Kind::Seq,
                bytes: BytesRange { start: 1, end: 6 },
                info: Info::default(),
                source: source.clone(),
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
                kind: Kind::Root,
                bytes: BytesRange::empty(0),
                info: Info {
                    subject: Some(Subject::Root),
                },
                source: source.clone(),
            },
            Block {
                id: 1024,
                found_at: 1,
                kind: Kind::Seq,
                bytes: BytesRange { start: 0, end: 3 },
                info: Info::default(),
                source: source.clone(),
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
                kind: Kind::Root,
                bytes: BytesRange::empty(0),
                info: Info {
                    subject: Some(Subject::Root),
                },
                source: source.clone(),
            },
            Block {
                id: 1024,
                found_at: 1,
                kind: Kind::Seq,
                bytes: BytesRange { start: 0, end: 6 },
                info: Info::default(),
                source: source.clone(),
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                kind: Kind::Space,
                bytes: BytesRange { start: 6, end: 7 },
                info: Info::default(),
                source: source.clone(),
            },
            Block {
                id: 1024 * 3,
                found_at: 3,
                kind: Kind::Seq,
                bytes: BytesRange { start: 7, end: 13 },
                info: Info::default(),
                source: source.clone(),
            },
            Block {
                id: 1024 * 4,
                found_at: 4,
                kind: Kind::Space,
                bytes: BytesRange { start: 13, end: 14 },
                info: Info::default(),
                source: source.clone(),
            },
            Block {
                id: 1024 * 5,
                found_at: 5,
                kind: Kind::Seq,
                bytes: BytesRange { start: 14, end: 20 },
                info: Info::default(),
                source: source.clone(),
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
                kind: Kind::Root,
                bytes: BytesRange::empty(0),
                info: Info {
                    subject: Some(Subject::Root),
                },
                source: source.clone(),
            },
            Block {
                id: 1024,
                found_at: 1,
                kind: Kind::Seq,
                bytes: BytesRange { start: 0, end: 3 },
                info: Info::default(),
                source: source.clone(),
            },
            Block {
                id: 1024 * 2,
                found_at: 2,
                kind: Kind::Colon,
                bytes: BytesRange { start: 3, end: 4 },
                info: Info::default(),
                source: source.clone(),
            },
            Block {
                id: 1024 * 3,
                found_at: 3,
                kind: Kind::Space,
                bytes: BytesRange { start: 4, end: 5 },
                info: Info::default(),
                source: source.clone(),
            },
            Block {
                id: 1024 * 4,
                found_at: 4,
                kind: Kind::Seq,
                bytes: BytesRange { start: 5, end: 7 },
                info: Info::default(),
                source: source.clone(),
            },
        ];
        assert_eq!(actual, expected);
    }
}
