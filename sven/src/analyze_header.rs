use crate::{
    additive::Additive,
    block::{Block, Status, Val},
    bytes::Bytes,
    domain::Domain,
    paper::Paper,
};

pub fn analyze_header(blocks: &mut Vec<Block>) {
    //
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

    mod general {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn a01() {
            let commit = ": fix me";
            let actual = with_commit(commit);

            // 1. missing type
            //      unless first other word closely matches our types (either specified in config
            //          or inferred from git commit history)
            //
            // 2. misplaced type
            //      unexpected space? (which comes after "fix")
            //
            // how to decide which one has priority or makes more sense?
            //
            // in this case we choose (2) because fix actually mathces our config
            // otherwise we would have choosen (1) because it's less steps for fixing

            assert_eq!(1, 2);
        }

        #[test]
        fn a02() {
            let commit = " :fix me";
            let actual = with_commit(commit);

            // 1. missing type
            //      missing colon
            //          unexpected colon
            //      misplaced colon
            //
            // * no unexpected space after misplaced colon path because we have
            //      "fix" come right after the ":" (portal source)

            // 2. misplaced type
            //      missing colon
            //          unexpected colon
            //      misplaced colon
            //
            //      + space
            //      ? colon
            //      & type
            //      ? space

            assert_eq!(1, 2);
        }
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
