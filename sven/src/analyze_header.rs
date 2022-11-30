use crate::{
    additive::Additive,
    block::{Block, Val},
    bytes::Bytes,
    domain::Domain,
    paper::Paper,
};

pub fn analyze_header(blocks: &mut Vec<Block>) {
    let mut paper = Paper::new();
    paper.root.found_at = blocks.first().unwrap().id;
    let mut desc_start_i = 0;

    for (i, block) in blocks.iter_mut().enumerate() {
        match block.val {
            Val::Seq => {
                if paper.kind.is_missing() {
                    paper.kind.found_at = block.id;
                    paper.kind.missing = false;
                    block.domain = Domain::Type;
                    desc_start_i = i + 1;
                }
            }
            Val::Colon => {
                if paper.colon.is_missing() {
                    paper.colon.found_at = block.id;
                    paper.colon.missing = false;
                    block.domain = Domain::Colon;
                    desc_start_i = i + 1;
                }
            }
            Val::Space => {
                if paper.space.is_missing() {
                    paper.space.found_at = block.id;
                    paper.space.missing = false;
                    block.domain = Domain::Space;
                    desc_start_i = i + 1;
                }
            }
            _ => {}
        }
    }

    match blocks.get_mut(desc_start_i..) {
        Some(desc) => {
            if !desc.is_empty() {
                let (first, last) = (desc.first().unwrap(), desc.last().unwrap());
                let repackaged_desc = Block {
                    id: first.id,
                    val: Val::Seq,
                    domain: Domain::Desc,
                    bytes: Some(Bytes::new(
                        first.bytes.unwrap().start(),
                        last.bytes.unwrap().end(),
                    )),
                };
                paper.desc.found_at = first.id;
                paper.desc.missing = false;
                blocks.drain(desc_start_i..);
                blocks.push(repackaged_desc);
            }
        }
        None => {}
    };

    paper.build_map();

    if paper.colon.is_missing() {
        blocks.push(Block {
            id: paper.colon.found_at,
            val: Val::Colon,
            domain: Domain::Colon,
            bytes: None,
        });
    }
    if paper.space.is_missing() {
        blocks.push(Block {
            id: paper.space.found_at,
            val: Val::Space,
            domain: Domain::Space,
            bytes: None,
        });
    }
    if paper.desc.is_missing() {
        blocks.push(Block {
            id: paper.desc.found_at,
            val: Val::Seq,
            domain: Domain::Desc,
            bytes: None,
        });
    }

    blocks.sort();

    println!("{:#?}", blocks);
    println!("{:#?}", paper);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bytes::Bytes, weak_commit::WeakCommit};
    use pretty_assertions::assert_eq;

    #[inline]
    fn with_commit(commit: &str) -> Vec<Block> {
        println!("commit {:?}", commit);
        let mut w = WeakCommit::parse(commit).unwrap();
        analyze_header(&mut w.header);
        w.header
    }

    mod missing {
        use crate::domain::Domain;

        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn colon() {
            let commit = "fix me";
            let actual = with_commit(commit);
            assert_eq!(actual[2].domain, Domain::Colon);
        }

        #[test]
        fn colon_2() {
            let commit = "fix";
            let actual = with_commit(commit);
            assert_eq!(actual[2].domain, Domain::Colon);
            assert_eq!(1, 2);
        }
    }

    #[test]
    fn perfect_reference() {
        let commit = "fix(refs)!: a simple fix";
    }
}
