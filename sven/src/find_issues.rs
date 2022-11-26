use crate::{find_header_issues::find_header_issues, report::Report, weak_commit::WeakCommit};
use anyhow::Result;
use std::collections::HashMap;

pub fn find_issues(commit: &str) -> Result<Report> {
    let weak_commit = WeakCommit::parse(commit)?;

    let header_issues = find_header_issues(&weak_commit.header);

    Ok(Report {
        header: header_issues,
        shape: (),
        footers: HashMap::new(),
    })
}
