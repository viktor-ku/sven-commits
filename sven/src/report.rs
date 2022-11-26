use crate::{footer_issue::footer, header_issue::header};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Report {
    pub header: Vec<header::Issue>,
    pub shape: (),
    pub footers: HashMap<usize, Vec<footer::Issue>>,
}
