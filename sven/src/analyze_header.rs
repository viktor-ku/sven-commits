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
    use super::*;
    use crate::{block_factory::BlockFactory, weak_commit::WeakCommit};
    use pretty_assertions::assert_eq;

    #[inline]
    fn with_commit(config: Config, commit: &str) -> Vec<Block> {
        println!("commit {:?}", commit);
        let w = WeakCommit::parse(commit).unwrap();
        analyze_header(commit, config, w.header)
    }

    /// "one" does not match any expected type value
    #[test]
    fn a01() {
        let blocks = with_commit(
            Config {
                known_type: KnownType::AnyFirstSeq,
            },
            "one two three",
        );
        assert_eq!(1, 2);
    }
}
