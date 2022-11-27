use crate::domain::Domain;
use std::{cmp::Ordering, fmt::Debug};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd)]
pub struct Pencil {
    pub domain: Domain,
    pub found_at: Option<usize>,
    pub next: Option<Domain>,
    pub prev: Option<Domain>,
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
        self.found_at.is_none()
    }
}
