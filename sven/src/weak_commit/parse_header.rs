use super::{bytes_range::BytesRange, CRule, CommitParser};
use crate::{
    additive::Additive,
    block::{Block, BlockKind},
};
use anyhow::Result;
use pest::Parser;

pub fn parse_header(header: &str) -> Result<Vec<Block>> {
    let mut v = Vec::new();
    let mut word_bytes = 0;
    let mut at = Additive::new();
    let mut id = Additive {
        step: 1024,
        val: 1024,
    };

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
                                    at: at.stamp(),
                                    kind: BlockKind::Seq,
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
                            v.push(Block {
                                id: id.stamp(),
                                at: at.stamp(),
                                kind: BlockKind::OpenBracket,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
                            });
                        }
                        CRule::TokenCloseBracket => {
                            v.push(Block {
                                id: id.stamp(),
                                at: at.stamp(),
                                kind: BlockKind::CloseBracket,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
                            });
                        }
                        CRule::TokenExclMark => {
                            v.push(Block {
                                id: id.stamp(),
                                at: at.stamp(),
                                kind: BlockKind::ExclMark,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
                            });
                        }
                        CRule::TokenColon => {
                            v.push(Block {
                                id: id.stamp(),
                                at: at.stamp(),
                                kind: BlockKind::Colon,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
                            });
                        }
                        CRule::TokenWhitespace => {
                            v.push(Block {
                                id: id.stamp(),
                                at: at.stamp(),
                                kind: BlockKind::Space,
                                bytes: BytesRange {
                                    start: span.start(),
                                    end: span.end(),
                                },
                                #[cfg(debug_assertions)]
                                source: header.to_string(),
                            });
                        }
                        CRule::TokenEOL => {
                            v.push(Block {
                                id: id.stamp(),
                                at: at.stamp(),
                                kind: BlockKind::EOL,
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
                v.push(Block {
                    id: id.stamp(),
                    at: at.stamp(),
                    kind: BlockKind::Seq,
                    bytes: BytesRange {
                        start: token.bytes.end,
                        end: token.bytes.end + word_bytes,
                    },
                    #[cfg(debug_assertions)]
                    source: header.to_string(),
                });
            }
            None => v.push(Block {
                id: id.stamp(),
                at: at.stamp(),
                kind: BlockKind::Seq,
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
            Block {
                id: 1024,
                at: 0,
                kind: BlockKind::Seq,
                bytes: BytesRange { start: 0, end: 3 },
                source: source.clone(),
            },
            Block {
                id: 1024 * 2,
                at: 1,
                kind: BlockKind::EOL,
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
            Block {
                id: 1024,
                at: 0,
                kind: BlockKind::Space,
                bytes: BytesRange { start: 0, end: 1 },
                source: source.clone(),
            },
            Block {
                id: 1024 * 2,
                at: 1,
                kind: BlockKind::Seq,
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
        let expected = vec![Block {
            id: 1024,
            at: 0,
            kind: BlockKind::Seq,
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
            Block {
                id: 1024,
                at: 0,
                kind: BlockKind::Seq,
                bytes: BytesRange { start: 0, end: 6 },
                source: source.clone(),
            },
            Block {
                id: 1024 * 2,
                at: 1,
                kind: BlockKind::Space,
                bytes: BytesRange { start: 6, end: 7 },
                source: source.clone(),
            },
            Block {
                id: 1024 * 3,
                at: 2,
                kind: BlockKind::Seq,
                bytes: BytesRange { start: 7, end: 13 },
                source: source.clone(),
            },
            Block {
                id: 1024 * 4,
                at: 3,
                kind: BlockKind::Space,
                bytes: BytesRange { start: 13, end: 14 },
                source: source.clone(),
            },
            Block {
                id: 1024 * 5,
                at: 4,
                kind: BlockKind::Seq,
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
            Block {
                id: 1024,
                at: 0,
                kind: BlockKind::Seq,
                bytes: BytesRange { start: 0, end: 3 },
                source: source.clone(),
            },
            Block {
                id: 1024 * 2,
                at: 1,
                kind: BlockKind::Colon,
                bytes: BytesRange { start: 3, end: 4 },
                source: source.clone(),
            },
            Block {
                id: 1024 * 3,
                at: 2,
                kind: BlockKind::Space,
                bytes: BytesRange { start: 4, end: 5 },
                source: source.clone(),
            },
            Block {
                id: 1024 * 4,
                at: 3,
                kind: BlockKind::Seq,
                bytes: BytesRange { start: 5, end: 7 },
                source: source.clone(),
            },
        ];
        assert_eq!(actual, expected);
    }
}
