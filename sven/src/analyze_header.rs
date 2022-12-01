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

    if paper.kind.is_missing() {
        blocks.push(Block {
            id: paper.kind.found_at,
            val: Val::Seq,
            domain: Domain::Type,
            bytes: None,
        });
    }
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
    use crate::{block_factory::BlockFactory, weak_commit::WeakCommit};
    use pretty_assertions::assert_eq;

    #[inline]
    fn with_commit(commit: &str) -> Vec<Block> {
        println!("commit {:?}", commit);
        let mut w = WeakCommit::parse(commit).unwrap();
        analyze_header(&mut w.header);
        w.header
    }

    mod missing {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn kind() {
            let commit = ": ";
            let actual = with_commit(commit);

            let f = {
                let mut f = BlockFactory::new();
                f.kind_missing(0, 512)
                    .colon(1)
                    .space(2)
                    .desc_missing(2, 512);
                f
            };

            assert_eq!(f.blocks, actual);
        }

        #[test]
        fn space() {
            let commit = "type:desc";
            let actual = with_commit(commit);

            let f = {
                let mut f = BlockFactory::new();
                f.kind(1, "type")
                    .colon(2)
                    .space_missing(2, 512)
                    .desc(3, "desc");
                f
            };

            assert_eq!(f.blocks, actual);
        }

        #[test]
        fn desc() {
            let commit = "type: ";
            let actual = with_commit(commit);

            let f = {
                let mut f = BlockFactory::new();
                f.kind(1, "type").colon(2).space(3).desc_missing(3, 512);
                f
            };

            assert_eq!(f.blocks, actual);
        }

        #[test]
        fn colon() {
            let commit = "fix me";
            let actual = with_commit(commit);

            let f = {
                let mut f = BlockFactory::new();
                f.kind(1, "fix")
                    .colon_missing(1, 512)
                    .space(2)
                    .desc(3, "me");
                f
            };

            assert_eq!(f.blocks, actual);
        }
    }
}
