use crate::{
    block::{Block, Val},
    bytes::Bytes,
    domain::Domain,
};

#[derive(Debug, Clone)]
pub struct BlockFactory {
    id: usize,
    pub blocks: Vec<Block>,
    end: usize,
}

impl BlockFactory {
    const C_STEP: usize = 1024;

    pub fn new() -> Self {
        Self {
            id: 0,
            end: 0,
            blocks: vec![Block {
                id: Some(0),
                val: Val::Root,
                bytes: None,
                domain: Domain::Root,
            }],
        }
    }

    #[inline]
    pub fn next_id(&mut self, mul: usize) {
        self.id = Self::C_STEP * mul;
    }

    pub fn kind(&mut self, add_mul: usize, val: &str) -> &mut Self {
        let val_bytes_len = val.as_bytes().len();
        self.next_id(add_mul);

        self.blocks.push(Block {
            id: Some(self.id),
            val: Val::Seq,
            domain: Domain::Type,
            bytes: Some(Bytes::new(self.end, self.end + val_bytes_len)),
        });

        self.end += val_bytes_len;
        self
    }

    pub fn space(&mut self, add_mul: usize) -> &mut Self {
        let bytes = Bytes::single(self.end);
        self.end = bytes.end();
        self.next_id(add_mul);

        self.blocks.push(Block {
            id: Some(self.id),
            val: Val::Space,
            domain: Domain::Space,
            bytes: Some(bytes),
        });

        self
    }

    pub fn desc(&mut self, add_mul: usize, val: &str) -> &mut Self {
        let val_bytes_len = val.as_bytes().len();
        self.next_id(add_mul);

        self.blocks.push(Block {
            id: Some(self.id),
            val: Val::Seq,
            domain: Domain::Desc,
            bytes: Some(Bytes::new(self.end, self.end + val_bytes_len)),
        });

        self.end += val_bytes_len;
        self
    }

    pub fn missing_colon(&mut self, add: usize) -> &mut Self {
        self.id += add;

        self.blocks.push(Block {
            id: Some(self.id),
            val: Val::Colon,
            domain: Domain::Colon,
            bytes: None,
        });

        self
    }
}
