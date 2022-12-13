use std::collections::HashSet;

#[derive(Debug)]
pub enum KnownType {
    AnyFirstSeq,

    /// Seq from the input is expected to strictly
    /// match predefined config (PartialEq)
    Strict(HashSet<String>),
}

#[derive(Debug)]
pub struct Config {
    pub known_type: KnownType,
}
