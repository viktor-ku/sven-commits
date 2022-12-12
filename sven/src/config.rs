use std::collections::HashSet;

#[derive(Debug)]
pub enum KnownType {
    AnyFirstSeq,
    Set(HashSet<String>),
}

#[derive(Debug)]
pub struct Config {
    pub known_type: KnownType,
}
