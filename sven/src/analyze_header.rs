use crate::{
    additive::Additive,
    block::{Block, Status, Val},
    bytes::Bytes,
    domain::Domain,
    paper::Paper,
};

pub fn analyze_header(blocks: &mut Vec<Block>) {
    let mut paper = Paper::new();
    let mut desc_start_i = 1;

    for (i, block) in blocks.iter_mut().enumerate() {
        match block.val {
            Val::Seq => {
                if paper.kind.is_missing() {
                    paper.kind.found_at = block.id;
                    paper.kind.missing = false;
                    block.domain = Domain::Type;
                    block.status = Status::Settled;
                    desc_start_i = i + 1;
                }
            }
            Val::Colon => {
                if paper.colon.is_missing() {
                    paper.colon.found_at = block.id;
                    paper.colon.missing = false;
                    block.domain = Domain::Colon;
                    block.status = Status::Settled;
                    desc_start_i = i + 1;
                }
            }
            Val::Space => {
                if paper.space.is_missing() {
                    paper.space.found_at = block.id;
                    paper.space.missing = false;
                    block.status = Status::Settled;
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
                    status: Status::Settled,
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
            status: Status::Missing,
        });
    }
    if paper.colon.is_missing() {
        blocks.push(Block {
            id: paper.colon.found_at,
            val: Val::Colon,
            domain: Domain::Colon,
            bytes: None,
            status: Status::Missing,
        });
    }
    if paper.space.is_missing() {
        blocks.push(Block {
            id: paper.space.found_at,
            val: Val::Space,
            domain: Domain::Space,
            bytes: None,
            status: Status::Missing,
        });
    }
    if paper.desc.is_missing() {
        blocks.push(Block {
            id: paper.desc.found_at,
            val: Val::Seq,
            domain: Domain::Desc,
            bytes: None,
            status: Status::Missing,
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
        fn scope_val() {
            let commit = "fix(): me";
            let actual = with_commit(commit);

            let f = {
                let mut f = BlockFactory::new();
                f.kind(1_000, "fix")
                    .scope_ob(2_000)
                    .scope_val_missing(2_500)
                    .scope_cb(3_000)
                    .colon(4_000)
                    .space(5_000)
                    .desc(6_000, "me");
                f
            };

            assert_eq!(f.blocks, actual);
        }

        #[test]
        fn scope_cb() {
            let commit = "fix(scope: me";
            let actual = with_commit(commit);

            let f = {
                let mut f = BlockFactory::new();
                f.kind(1_000, "fix")
                    .scope_ob(2_000)
                    .scope_val(3_000, "scope")
                    .scope_cb(3_500)
                    .colon(4_000)
                    .space(5_000)
                    .desc(6_000, "me");
                f
            };

            assert_eq!(f.blocks, actual);
        }

        #[test]
        fn all() {
            let commit = "";
            let actual = with_commit(commit);

            let f = {
                let mut f = BlockFactory::new();
                f.kind_missing(200)
                    .colon_missing(400)
                    .space_missing(600)
                    .desc_missing(800);
                f
            };

            assert_eq!(f.blocks, actual);
        }

        #[test]
        fn kind() {
            let commit = ": ";
            let actual = with_commit(commit);

            let f = {
                let mut f = BlockFactory::new();
                f.kind_missing(500)
                    .colon(1_000)
                    .space(2_000)
                    .desc_missing(2_500);
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
                f.kind(1_000, "type")
                    .colon(2_000)
                    .space_missing(2_500)
                    .desc(3_000, "desc");
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
                f.kind(1_000, "type")
                    .colon(2_000)
                    .space(3_000)
                    .desc_missing(3_500);
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
                f.kind(1_000, "fix")
                    .colon_missing(1_500)
                    .space(2_000)
                    .desc(3_000, "me");
                f
            };

            assert_eq!(f.blocks, actual);
        }
    }
}
