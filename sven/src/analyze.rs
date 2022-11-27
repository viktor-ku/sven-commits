use crate::{analyze_header::analyze_header, report::Report, weak_commit::WeakCommit};
use anyhow::Result;
use std::collections::HashMap;

pub fn analyze(commit: &str) -> Result<Report> {
    let mut weak_commit = WeakCommit::parse(commit)?;

    analyze_header(&mut weak_commit.header);

    Ok(Report {
        header: Vec::new(),
        shape: (),
        footers: HashMap::new(),
    })
}
