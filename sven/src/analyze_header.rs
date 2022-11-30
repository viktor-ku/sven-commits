use crate::{
    additive::Additive,
    block::{Block, Val},
    paper::Paper,
};
use std::collections::BTreeSet;

pub fn analyze_header(blocks: &mut BTreeSet<Block>) {
    todo!()
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
    }
}
