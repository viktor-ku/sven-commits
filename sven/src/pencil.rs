use crate::subject::Subject;
use std::{cmp::Ordering, fmt::Debug};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd)]
pub struct Pencil {
    pub subject: Subject,
    pub found_at: Option<usize>,
    pub next: Option<Subject>,
    pub prev: Option<Subject>,
}

impl Ord for Pencil {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.found_at, other.found_at) {
            (Some(me_at), Some(other_at)) => me_at.cmp(&other_at),
            _ => self.subject.cmp(&other.subject),
        }
    }
}

impl Pencil {
    #[inline]
    pub fn is_missing(&self) -> bool {
        self.found_at.is_none()
    }
}
