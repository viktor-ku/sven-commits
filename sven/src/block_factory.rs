use crate::{
    block::{Block, Val},
    bytes::Bytes,
    domain::Domain,
};

#[derive(Debug, Clone)]
pub struct BlockFactory {
    pub blocks: Vec<Block>,
    end: usize,
}

impl BlockFactory {
    const C_STEP: usize = 1024;

    pub fn new() -> Self {
        Self {
            end: 0,
            blocks: vec![Block {
                id: Some(0),
                val: Val::Root,
                bytes: None,
                domain: Domain::Root,
            }],
        }
    }

    pub fn kind(&mut self, mul: usize, val: &str) -> &mut Self {
        let val_bytes_len = val.as_bytes().len();

        self.blocks.push(Block {
            id: Some(Self::C_STEP * mul),
            val: Val::Seq,
            domain: Domain::Type,
            bytes: Some(Bytes::new(self.end, self.end + val_bytes_len)),
        });

        self.end += val_bytes_len;
        self
    }

    pub fn colon(&mut self, mul: usize) -> &mut Self {
        let bytes = Bytes::single(self.end);
        self.end = bytes.end();

        self.blocks.push(Block {
            id: Some(Self::C_STEP * mul),
            val: Val::Colon,
            domain: Domain::Colon,
            bytes: Some(bytes),
        });

        self
    }

    pub fn space(&mut self, mul: usize) -> &mut Self {
        let bytes = Bytes::single(self.end);
        self.end = bytes.end();

        self.blocks.push(Block {
            id: Some(Self::C_STEP * mul),
            val: Val::Space,
            domain: Domain::Space,
            bytes: Some(bytes),
        });

        self
    }

    pub fn desc(&mut self, mul: usize, val: &str) -> &mut Self {
        let val_bytes_len = val.as_bytes().len();

        self.blocks.push(Block {
            id: Some(Self::C_STEP * mul),
            val: Val::Seq,
            domain: Domain::Desc,
            bytes: Some(Bytes::new(self.end, self.end + val_bytes_len)),
        });

        self.end += val_bytes_len;
        self
    }

    pub fn colon_missing(&mut self, base: usize, add: usize) -> &mut Self {
        self.blocks.push(Block {
            id: Some(base * 1024 + add),
            val: Val::Colon,
            domain: Domain::Colon,
            bytes: None,
        });

        self
    }

    pub fn space_missing(&mut self, base: usize, add: usize) -> &mut Self {
        self.blocks.push(Block {
            id: Some(base * 1024 + add),
            val: Val::Space,
            domain: Domain::Space,
            bytes: None,
        });

        self
    }

    pub fn desc_missing(&mut self, base: usize, add: usize) -> &mut Self {
        self.blocks.push(Block {
            id: Some(base * 1024 + add),
            val: Val::Seq,
            domain: Domain::Desc,
            bytes: None,
        });

        self
    }
}
