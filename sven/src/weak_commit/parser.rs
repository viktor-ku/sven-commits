#[derive(Parser)]
#[grammar = "./weak_commit/grammar.pest"] // relative to src
pub struct CommitParser;

pub type CRule = Rule;
