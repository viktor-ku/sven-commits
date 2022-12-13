use crate::{
    analyze_header::analyze_header,
    config::{Config, TypeRule},
    report::Report,
    weak_commit::WeakCommit,
};
use anyhow::Result;
use std::collections::HashMap;

pub fn analyze(commit: &str) -> Result<Report> {
    let weak_commit = WeakCommit::parse(commit)?;

    let config = Config {
        type_rule: TypeRule::AnyFirstSeq,
    };
    analyze_header(commit, &config, weak_commit.header);

    Ok(Report {
        header: Vec::new(),
        shape: (),
        footers: HashMap::new(),
    })
}
