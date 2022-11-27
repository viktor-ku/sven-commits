use crate::footer_issue::footer;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Report {
    pub header: Vec<()>,
    pub shape: (),
    pub footers: HashMap<usize, Vec<footer::Issue>>,
}
