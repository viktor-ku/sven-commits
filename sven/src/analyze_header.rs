use crate::{
    block::{Block, Status, Val},
    config::{Config, TypeRule},
    domain::Domain,
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Portal {
    found_at: usize,
    pointing_at: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Portals {
    pub kind: Option<Portal>,
    pub colon: Option<Portal>,
    pub space: Option<Portal>,
}

impl Portals {
    pub fn is_empty(&self) -> bool {
        self.kind.is_none() && self.colon.is_none() && self.space.is_none()
    }
}

/// Analyse header blocks returning an optimal solution
/// that fulfills the conventional commit specification
pub fn analyze_header(commit: &str, config: &Config, blocks: Vec<Block>) -> Vec<Block> {
    let mut all_solutions = Vec::new();

    find_solutions(commit, config, blocks, &mut all_solutions);

    pick_solution(all_solutions)
}

fn pick_solution(all: Vec<Vec<Block>>) -> Vec<Block> {
    if all.is_empty() {
        Vec::new()
    } else if all.len() == 1 {
        let v = &all[0];
        v.clone()
    } else {
        todo!()
    }
}

fn find_solutions(
    commit: &str,
    config: &Config,
    candidate: Vec<Block>,
    solutions: &mut Vec<Vec<Block>>,
) {
    let q = vec![Domain::Type, Domain::Colon, Domain::Space, Domain::Desc];
    let mut q = q.iter().peekable();

    let mut portals = Portals::default();

    let mut candidate = candidate;
    let blocks_iter = {
        let mut iter = candidate.iter_mut().enumerate();
        iter.next(); // skip root
        iter
    };

    macro_rules! try_missing {
        ($i:expr, $val:expr) => {
            let block = Block {
                id: None,
                val: $val,
                domain: $val.into(),
                bytes: None,
                status: Status::Missing,
            };

            let mut alternative = candidate.clone();
            alternative.insert($i, block);

            find_solutions(commit, config, alternative, solutions);
        };
    }
    macro_rules! try_misplaced {
        ($i:expr, $val:expr) => {
            let block = Block {
                id: None,
                val: $val,
                domain: $val.into(),
                bytes: None,
                status: Status::Portal(None),
            };

            let mut alternative = candidate.clone();
            alternative.insert($i, block);

            find_solutions(commit, config, alternative, solutions);
        };
    }

    for (i, block) in blocks_iter {
        let q_domain = q.peek();

        match block.status {
            Status::Portal(dest) => {
                match dest {
                    Some(_) => {
                        // if we get here it means we already set destination
                        // for the current portal? but when?
                        todo!()
                    }
                    None => match block.domain {
                        Domain::Type => {
                            portals.kind = Some(Portal {
                                found_at: i,
                                pointing_at: None,
                            });
                        }
                        Domain::Colon => {
                            portals.colon = Some(Portal {
                                found_at: i,
                                pointing_at: None,
                            });
                        }
                        Domain::Space => {
                            portals.space = Some(Portal {
                                found_at: i,
                                pointing_at: None,
                            });
                        }
                        _ => {}
                    },
                };
            }
            _ => {}
        };

        match q_domain {
            Some(&q_domain) => match q_domain {
                Domain::Type => {
                    if is_type(&config.type_rule, &block, commit) {
                        q.next();
                        block.domain = Domain::Type;
                        if block.status == Status::Unsigned {
                            block.status = Status::Settled;
                        }
                    } else {
                        try_missing!(i, Val::Seq);
                        try_misplaced!(i, Val::Seq);
                        return;
                    }
                }
                Domain::Colon => {
                    if block.val == Val::Colon {
                        q.next();
                        block.domain = Domain::Colon;
                        if block.status == Status::Unsigned {
                            block.status = Status::Settled;
                        }
                    } else {
                        try_missing!(i, Val::Colon);
                        try_misplaced!(i, Val::Colon);
                        return;
                    }
                }
                Domain::Space => {
                    if block.val == Val::Space {
                        q.next();
                        block.domain = Domain::Space;
                        if block.status == Status::Unsigned {
                            block.status = Status::Settled;
                        }
                    } else {
                        try_missing!(i, Val::Space);
                        try_misplaced!(i, Val::Space);
                        return;
                    }
                }
                Domain::Desc => {
                    break;
                }
                _ => todo!(),
            },
            None => {
                todo!()
            }
        }
    }

    // TODO: check if queue is not empty then we have some blocks missing still

    if !portals.is_empty() {
        return;
    }

    // when we reach here, assume _a_ possible solution found
    solutions.push(candidate);
}

fn is_type(expected_type: &TypeRule, actual_block: &Block, commit: &str) -> bool {
    match expected_type {
        TypeRule::AnyFirstSeq => match actual_block.val {
            Val::Seq => true,
            _ => false,
        },
        TypeRule::Strict(set) => match (actual_block.domain, actual_block.val) {
            (Domain::Type, _) => true,
            (_, Val::Seq) => match actual_block.capture(commit) {
                Some(val) => set.get(val).is_some(),
                None => false,
            },
            _ => false,
        },
        TypeRule::Like(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{block_factory::BlockFactory, weak_commit::WeakCommit};
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    #[inline]
    fn with_commit(config: &Config, commit: &str) -> Vec<Block> {
        println!("commit {:?}", commit);
        let w = WeakCommit::parse(commit).unwrap();
        analyze_header(commit, &config, w.header)
    }

    #[test]
    fn just_colon_is_missing_when_type_is_first_seq() {
        let blocks = with_commit(
            &Config {
                type_rule: TypeRule::AnyFirstSeq,
            },
            "one two three",
        );

        let f = {
            let mut f = BlockFactory::new();
            f.kind(1_000, "one").colon_missing(1_500).space(2_000);
            f
        };

        assert_eq!(f.blocks, blocks[..f.end_blocks]);
    }

    #[test]
    fn just_colon_is_missing_when_type_is_known() {
        let blocks = with_commit(
            &Config {
                type_rule: TypeRule::Strict(HashSet::from_iter(["fix".to_string()])),
            },
            "fix two three",
        );

        let f = {
            let mut f = BlockFactory::new();
            f.kind(1_000, "fix").colon_missing(1_500).space(2_000);
            f
        };

        assert_eq!(f.blocks, blocks[..f.end_blocks]);
    }

    #[test]
    fn only_desc_found_when_could_not_find_type() {
        let blocks = with_commit(
            &Config {
                type_rule: TypeRule::Strict(HashSet::from_iter(["fix".to_string()])),
            },
            "one two three",
        );

        let f = {
            let mut f = BlockFactory::new();
            f.kind_missing(250).colon_missing(500).space_missing(750);
            f
        };

        assert_eq!(f.blocks, blocks[..f.end_blocks]);
    }
}
