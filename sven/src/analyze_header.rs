use crate::{
    block::{Block, Status, Val},
    config::{Config, TypeRule},
    domain::Domain,
};

#[derive(Debug, Default)]
pub struct Portal {
    pub found_at: usize,
    pub pointing_at: Option<usize>,
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

impl Portal {
    pub fn is_connected(&self) -> bool {
        self.pointing_at.is_some()
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Candidate {
    pub weight: usize,
    pub solution: Vec<Block>,
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

/// Analyse header blocks returning an optimal solution
/// that fulfills the conventional commit specification
pub fn analyze_header(commit: &str, config: &Config, blocks: Vec<Block>) -> Vec<Block> {
    let mut all_solutions = Vec::new();

    find_solutions(commit, config, blocks, &mut all_solutions);

    pick_solution(all_solutions)
}

fn pick_solution(all: Vec<Candidate>) -> Vec<Block> {
    if all.is_empty() {
        Vec::new()
    } else if all.len() == 1 {
        all[0].solution.clone()
    } else {
        let mut all = all;
        all.sort();
        all[0].solution.clone()
    }
}

fn find_solutions(
    commit: &str,
    config: &Config,
    candidate: Vec<Block>,
    solutions: &mut Vec<Candidate>,
) {
    let q = vec![Domain::Type, Domain::Colon, Domain::Space, Domain::Desc];
    let mut q = q.iter().peekable();

    let mut portals = Portals::default();
    let mut weight: usize = 0;

    let mut candidate = candidate;
    let blocks_iter = {
        let mut iter = candidate.iter_mut().enumerate();
        iter.next(); // skip root
        iter
    };

    macro_rules! try_missing {
        ($i:expr, $val:expr) => {
            let block = Block {
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
                val: $val,
                domain: $val.into(),
                bytes: None,
                status: Status::Misplaced,
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
                weight += 1;
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
            Status::Missing => {
                weight += 1;
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
                Domain::Desc => {}
                _ => todo!(),
            },
            None => {
                todo!()
            }
        }

        match block.val {
            Val::Colon => {
                if block.status == Status::Unsigned {
                    match &mut portals.colon {
                        Some(colon_portal) => {
                            if colon_portal.is_connected() {
                                block.status = Status::Extra;
                                weight += 1;
                            } else {
                                colon_portal.pointing_at = Some(i);
                                block.status = Status::Ref(colon_portal.found_at);
                                block.domain = Domain::Colon;
                            }
                        }
                        None => {
                            block.status = Status::Extra;
                            weight += 1;
                        }
                    }
                }
            }
            _ => {}
        };
    }

    if let Some(kind) = &portals.kind {
        if kind.is_connected() {
            candidate[kind.found_at].status = Status::Portal(Some(kind.pointing_at.unwrap()));
            portals.kind = None;
        }
    }
    if let Some(colon) = &portals.colon {
        if colon.is_connected() {
            candidate[colon.found_at].status = Status::Portal(Some(colon.pointing_at.unwrap()));
            portals.colon = None;
        }
    }
    if let Some(space) = &portals.space {
        if space.is_connected() {
            candidate[space.found_at].status = Status::Portal(Some(space.pointing_at.unwrap()));
            portals.colon = None;
        }
    }

    println!("{:#?}", solutions);

    if !portals.is_empty() {
        return;
    }

    // when we reach here, assume _a_ possible solution found
    solutions.push(Candidate {
        weight,
        solution: candidate,
    });
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
    fn it_works() {
        let blocks = with_commit(
            &Config {
                type_rule: TypeRule::Strict(HashSet::from_iter(["fix".to_string()])),
            },
            " :not me",
        );

        let f = {
            let mut f = BlockFactory::new();
            f.kind_missing().colon_misplaced().space().colon_ref();
            f
        };

        assert_eq!(f.blocks, blocks[..f.end_blocks]);
    }

    #[test]
    fn it_works_2() {
        let blocks = with_commit(
            &Config {
                type_rule: TypeRule::Strict(HashSet::from_iter(["fix".to_string()])),
            },
            " :fix me",
        );

        let f = {
            let mut f = BlockFactory::new();
            f.space_extra().colon_ref().kind("fix").colon_misplaced();
            f
        };

        assert_eq!(f.blocks, blocks[..f.end_blocks]);
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
            f.kind("one").colon_missing().space();
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
            f.kind("fix").colon_missing().space();
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
            f.kind_missing().colon_missing().space_missing();
            f
        };

        assert_eq!(f.blocks, blocks[..f.end_blocks]);
    }
}
