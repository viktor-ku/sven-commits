use std::collections::HashSet;

#[derive(Debug)]
pub enum TypeRule {
    /// Match first Seq from the input
    AnyFirstSeq,

    /// Seq from the input is expected to _strictly_
    /// match predefined set
    Strict(HashSet<String>),

    /// Seq from the input is expected to _roughly_
    /// match predefined set
    Like(HashSet<String>),
}

#[derive(Debug)]
pub struct Config {
    pub type_rule: TypeRule,
}
