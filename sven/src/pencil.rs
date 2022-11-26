use crate::subject::Subject;
use std::{cmp::Ordering, fmt::Debug};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pencil {
    pub subject: Subject,
    pub found_at: Option<usize>,
    pub next: Option<Subject>,
    pub prev: Option<Subject>,
}

impl PartialOrd for Pencil {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.found_at, other.found_at) {
            (Some(me_at), Some(other_at)) => {
                if me_at > other_at {
                    Some(Ordering::Greater)
                } else if me_at == other_at {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Less)
                }
            }
            _ => None,
        }
    }
}

impl Ord for Pencil {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => self.subject.cmp(&other.subject),
        }
    }
}

impl Pencil {
    #[inline]
    pub fn is_missing(&self) -> bool {
        self.found_at.is_none()
    }
}
