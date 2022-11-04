use pest::Parser;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct CommitParser;

pub fn commit_parser(commit: &str) -> () {
    let res = CommitParser::parse(Rule::Input, commit).unwrap();
    println!("{:#?}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        commit_parser("something?");
        assert_eq!(1, 2);
    }
}
