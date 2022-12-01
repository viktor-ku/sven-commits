use crate::{
    block::{Block, Status, Val},
    bytes::Bytes,
    domain::{Domain, Scope},
};

#[derive(Debug, Clone)]
pub struct BlockFactory {
    pub blocks: Vec<Block>,
    end: usize,
}

impl BlockFactory {
    pub fn new() -> Self {
        Self {
            end: 0,
            blocks: vec![Block {
                id: Some(0),
                val: Val::Root,
                bytes: None,
                domain: Domain::Root,
                status: Status::Settled,
            }],
        }
    }

    pub fn kind(&mut self, id: usize, val: &str) -> &mut Self {
        let val_bytes_len = val.as_bytes().len();

        self.blocks.push(Block {
            id: Some(id),
            val: Val::Seq,
            domain: Domain::Type,
            bytes: Some(Bytes::new(self.end, self.end + val_bytes_len)),
            status: Status::Settled,
        });

        self.end += val_bytes_len;
        self
    }

    pub fn scope_ob(&mut self, id: usize) -> &mut Self {
        let bytes = Bytes::single(self.end);
        self.end = bytes.end();

        self.blocks.push(Block {
            id: Some(id),
            val: Val::OpenBracket,
            domain: Domain::Scope(Scope::OpenBracket),
            bytes: Some(bytes),
            status: Status::Settled,
        });

        self
    }

    pub fn scope_val(&mut self, id: usize, val: &str) -> &mut Self {
        let val_bytes_len = val.as_bytes().len();

        self.blocks.push(Block {
            id: Some(id),
            val: Val::Seq,
            domain: Domain::Scope(Scope::Scope),
            bytes: Some(Bytes::new(self.end, self.end + val_bytes_len)),
            status: Status::Settled,
        });

        self.end += val_bytes_len;
        self
    }

    pub fn scope_cb(&mut self, id: usize) -> &mut Self {
        let bytes = Bytes::single(self.end);
        self.end = bytes.end();

        self.blocks.push(Block {
            id: Some(id),
            val: Val::CloseBracket,
            domain: Domain::Scope(Scope::CloseBracket),
            bytes: Some(bytes),
            status: Status::Settled,
        });

        self
    }

    pub fn colon(&mut self, id: usize) -> &mut Self {
        let bytes = Bytes::single(self.end);
        self.end = bytes.end();

        self.blocks.push(Block {
            id: Some(id),
            val: Val::Colon,
            domain: Domain::Colon,
            bytes: Some(bytes),
            status: Status::Settled,
        });

        self
    }

    pub fn space(&mut self, id: usize) -> &mut Self {
        let bytes = Bytes::single(self.end);
        self.end = bytes.end();

        self.blocks.push(Block {
            id: Some(id),
            val: Val::Space,
            domain: Domain::Space,
            bytes: Some(bytes),
            status: Status::Settled,
        });

        self
    }

    pub fn desc(&mut self, id: usize, val: &str) -> &mut Self {
        let val_bytes_len = val.as_bytes().len();

        self.blocks.push(Block {
            id: Some(id),
            val: Val::Seq,
            domain: Domain::Desc,
            bytes: Some(Bytes::new(self.end, self.end + val_bytes_len)),
            status: Status::Settled,
        });

        self.end += val_bytes_len;
        self
    }

    pub fn colon_missing(&mut self, id: usize) -> &mut Self {
        self.blocks.push(Block {
            id: Some(id),
            val: Val::Colon,
            domain: Domain::Colon,
            bytes: None,
            status: Status::Missing,
        });

        self
    }

    pub fn space_missing(&mut self, id: usize) -> &mut Self {
        self.blocks.push(Block {
            id: Some(id),
            val: Val::Space,
            domain: Domain::Space,
            bytes: None,
            status: Status::Missing,
        });

        self
    }

    pub fn desc_missing(&mut self, id: usize) -> &mut Self {
        self.blocks.push(Block {
            id: Some(id),
            val: Val::Seq,
            domain: Domain::Desc,
            bytes: None,
            status: Status::Missing,
        });

        self
    }

    pub fn kind_missing(&mut self, id: usize) -> &mut Self {
        self.blocks.push(Block {
            id: Some(id),
            val: Val::Seq,
            domain: Domain::Type,
            bytes: None,
            status: Status::Missing,
        });

        self
    }

    pub fn scope_ob_missing(&mut self, id: usize) -> &mut Self {
        self.blocks.push(Block {
            id: Some(id),
            val: Val::OpenBracket,
            domain: Domain::Scope(Scope::OpenBracket),
            bytes: None,
            status: Status::Missing,
        });

        self
    }

    pub fn scope_val_missing(&mut self, id: usize) -> &mut Self {
        self.blocks.push(Block {
            id: Some(id),
            val: Val::Seq,
            domain: Domain::Scope(Scope::Scope),
            bytes: None,
            status: Status::Missing,
        });

        self
    }

    pub fn scope_cb_missing(&mut self, id: usize) -> &mut Self {
        self.blocks.push(Block {
            id: Some(id),
            val: Val::CloseBracket,
            domain: Domain::Scope(Scope::CloseBracket),
            bytes: None,
            status: Status::Missing,
        });

        self
    }
}
