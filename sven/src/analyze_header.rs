use crate::{
    additive::Additive,
    block::{Block, Status, Val},
    bytes::Bytes,
    config::{Config, KnownType},
    domain::Domain,
    paper::Paper,
};

pub fn analyze_header(commit: &str, config: Config, blocks: Vec<Block>) -> Vec<Block> {
    // 1. we need to know if we have scope
    // 2. we need to know if we have !
    // 3. do we care where desc starts?

    Vec::new()
}

fn is_type(expected_type: &KnownType, actual_block: &Block, commit: &str) -> bool {
    match expected_type {
        KnownType::AnyFirstSeq => match actual_block.val {
            Val::Seq => true,
            _ => false,
        },
        KnownType::Set(set) => match actual_block.val {
            Val::Seq => {
                let val = actual_block.capture(commit).unwrap();

                set.get(val).is_some()
            }
            _ => false,
        },
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::{block_factory::BlockFactory, weak_commit::WeakCommit};
    use pretty_assertions::assert_eq;

    #[inline]
    fn with_commit(config: Config, commit: &str) -> Vec<Block> {
        println!("commit {:?}", commit);
        let w = WeakCommit::parse(commit).unwrap();
        analyze_header(commit, config, w.header)
    }

    #[test]
    fn just_colon_is_missing_when_type_is_first_seq() {
        let blocks = with_commit(
            Config {
                known_type: KnownType::AnyFirstSeq,
            },
            "one two three",
        );

        let f = {
            let mut f = BlockFactory::new();
            f.kind(1_000, "one")
                .colon_missing(1_500)
                .space(2_000)
                .desc(3_000, "two three");
            f
        };

        assert_eq!(f.blocks, blocks);
    }

    #[test]
    fn just_colon_is_missing_when_type_is_known() {
        let blocks = with_commit(
            Config {
                known_type: KnownType::Set(HashSet::from_iter(["fix".to_string()])),
            },
            "fix two three",
        );

        let f = {
            let mut f = BlockFactory::new();
            f.kind(1_000, "fix")
                .colon_missing(1_500)
                .space(2_000)
                .desc(3_000, "two three");
            f
        };

        assert_eq!(f.blocks, blocks);
    }

    #[test]
    fn only_desc_found_when_could_not_find_type() {
        let blocks = with_commit(
            Config {
                known_type: KnownType::Set(HashSet::from_iter(["fix".to_string()])),
            },
            "one two three",
        );

        let f = {
            let mut f = BlockFactory::new();
            f.kind_missing(250)
                .colon_missing(500)
                .space_missing(750)
                .desc(1_000, "one two three");
            f
        };

        assert_eq!(f.blocks, blocks);
    }
}
