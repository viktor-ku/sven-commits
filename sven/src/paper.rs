use std::{borrow::BorrowMut, cmp::Ordering, collections::BTreeSet, fmt::Debug};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Subject {
    Root,
    Kind,
    Colon,
    Space,
    Desc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pencil {
    pub id: usize,
    pub subject: Subject,
    pub found_at: Option<usize>,
    pub next: Option<usize>,
    pub prev: Option<usize>,
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
            None => self.id.cmp(&other.id),
        }
    }
}

impl Pencil {
    #[inline]
    pub fn is_missing(&self) -> bool {
        self.found_at.is_none()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paper {
    root: Pencil,
    pub kind: Pencil,
    pub colon: Pencil,
    pub space: Pencil,
    pub desc: Pencil,
}

impl Paper {
    pub fn new() -> Self {
        Self {
            root: Pencil {
                id: 0,
                subject: Subject::Root,
                found_at: None,
                next: None,
                prev: None,
            },
            kind: Pencil {
                id: 1,
                subject: Subject::Kind,
                found_at: None,
                next: None,
                prev: None,
            },
            colon: Pencil {
                id: 2,
                subject: Subject::Colon,
                found_at: None,
                next: None,
                prev: None,
            },
            space: Pencil {
                id: 3,
                subject: Subject::Space,
                found_at: None,
                next: None,
                prev: None,
            },
            desc: Pencil {
                id: 4,
                subject: Subject::Desc,
                found_at: None,
                next: None,
                prev: None,
            },
        }
    }

    pub fn build_map(&mut self) {
        let mut t: BTreeSet<Pencil> = BTreeSet::new();
        t.insert(self.root);
        t.insert(self.kind);
        t.insert(self.colon);
        t.insert(self.space);
        t.insert(self.desc);

        let mut prev: Option<usize> = None;
        let mut next: Option<Subject> = None;
        for one in t {
            match prev {
                Some(id) => match one.subject {
                    Subject::Root => {}
                    Subject::Kind => self.kind.prev = Some(id),
                    Subject::Colon => self.colon.prev = Some(id),
                    Subject::Space => self.space.prev = Some(id),
                    Subject::Desc => self.desc.prev = Some(id),
                },
                None => {}
            };
            prev = Some(one.id);

            match next {
                Some(subject) => match subject {
                    Subject::Root => self.root.next = Some(one.id),
                    Subject::Kind => self.kind.next = Some(one.id),
                    Subject::Colon => self.colon.next = Some(one.id),
                    Subject::Space => self.space.next = Some(one.id),
                    Subject::Desc => self.desc.next = Some(one.id),
                },
                None => {}
            }
            next = Some(one.subject);
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.kind.is_missing()
            && self.colon.is_missing()
            && self.space.is_missing()
            && self.desc.is_missing()
    }
}
