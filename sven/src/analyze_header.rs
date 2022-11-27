use crate::{
    additive::Additive,
    block::{self, Block},
    paper::Paper,
};
use std::collections::BTreeSet;

pub fn analyze_header(blocks: &mut BTreeSet<Block>) {
    let mut paper = Paper::new();
    let mut id = Additive::new();

    println!("blocks {:#?}", blocks);

    // find first occurences of every paper token, except the desc
    for block in blocks.iter() {
        match block.kind {
            block::Kind::Seq => {
                if paper.kind.is_missing() {
                    paper.kind.found_at = Some(block.found_at);
                }
            }
            block::Kind::Colon => {
                if paper.colon.is_missing() {
                    paper.colon.found_at = Some(block.found_at);
                }
            }
            block::Kind::Space => {
                if paper.space.is_missing() {
                    paper.space.found_at = Some(block.found_at);
                }
            }
            _ => {}
        }
    }
    // this is our first attempt at figuring out where the desc starts, that is
    // after the at most far token we found + 1
    let desc_start = [
        paper.kind.found_at,
        paper.colon.found_at,
        paper.space.found_at,
        paper.desc.found_at,
    ]
    .iter()
    .flatten()
    .max()
    .map(|x| x + 1);
    match desc_start {
        Some(desc_start) => {
            paper.desc.found_at = Some(desc_start);
        }
        None => {}
    };

    paper.build_map();

    println!("{:#?}", paper);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weak_commit::WeakCommit;
    use pretty_assertions::assert_eq;

    fn with_commit(commit: &str) -> Vec<Block> {
        println!("commit {:?}", commit);
        let mut w = WeakCommit::parse(commit).unwrap();
        analyze_header(&mut w.header);
        Vec::from_iter(w.header)
    }

    #[test]
    fn perfect_reference_no_issues() {
        let commit = r###"
fix(refs)!: a simple fix

Раз два три

This test proves utf8 works

Refs: #1001
BREAKING CHANGE: supports many footers
"###
        .trim_start();
        let actual = with_commit(commit);
        for one in actual {
            if one.info.subject.is_none() {
                panic!("everything should be related to conventional commit {:#?}", one);
            }
        }
    }
}
