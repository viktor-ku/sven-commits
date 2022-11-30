use crate::domain::Domain;
use std::{cmp::Ordering, fmt::Debug};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pencil {
    pub domain: Domain,
    pub found_at: Option<usize>,
    pub next: Option<Domain>,
    pub prev: Option<Domain>,
    pub missing: bool,
    pub missing_nth: usize,
    pub missing_total: usize,
}

impl PartialOrd for Pencil {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Pencil {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.found_at, other.found_at) {
            (Some(me_at), Some(other_at)) => me_at.cmp(&other_at),
            _ => self.domain.cmp(&other.domain),
        }
    }
}

impl Pencil {
    #[inline]
    pub fn is_missing(&self) -> bool {
        self.missing
    }
}
