use crate::{pencil::Pencil, subject::Subject};
use std::{collections::BTreeSet, fmt::Debug};

#[derive(PartialEq, Eq)]
pub struct Paper {
    root: Pencil,
    pub kind: Pencil,
    pub colon: Pencil,
    pub space: Pencil,
    pub desc: Pencil,
}

impl Paper {
    #[inline]
    pub fn new() -> Self {
        Self {
            root: Pencil {
                subject: Subject::Root,
                found_at: None,
                next: None,
                prev: None,
            },
            kind: Pencil {
                subject: Subject::Kind,
                found_at: None,
                next: None,
                prev: None,
            },
            colon: Pencil {
                subject: Subject::Colon,
                found_at: None,
                next: None,
                prev: None,
            },
            space: Pencil {
                subject: Subject::Space,
                found_at: None,
                next: None,
                prev: None,
            },
            desc: Pencil {
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

        let mut prev: Option<Subject> = None;
        let mut next: Option<Subject> = None;
        for one in t {
            if let Some(prev_subject) = prev {
                self.find_pencil_mut(one.subject).prev = Some(prev_subject);
            }
            prev = Some(one.subject);

            if let Some(next_subject) = next {
                self.find_pencil_mut(next_subject).next = Some(one.subject);
            }
            next = Some(one.subject);
        }

        debug_assert!(self.root.prev.is_none());
        debug_assert!(self.desc.next.is_none());
    }

    #[inline]
    pub fn find_pencil(&self, subject: Subject) -> &Pencil {
        match subject {
            Subject::Root => &self.root,
            Subject::Kind => &self.kind,
            Subject::Colon => &self.colon,
            Subject::Space => &self.space,
            Subject::Desc => &self.desc,
        }
    }

    #[inline]
    pub fn find_pencil_mut(&mut self, subject: Subject) -> &mut Pencil {
        match subject {
            Subject::Root => &mut self.root,
            Subject::Kind => &mut self.kind,
            Subject::Colon => &mut self.colon,
            Subject::Space => &mut self.space,
            Subject::Desc => &mut self.desc,
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

impl Debug for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[inline]
        fn at(at: Option<usize>) -> String {
            match at {
                Some(at) => format!("{}", at),
                None => "no".to_string(),
            }
        }

        writeln!(
            f,
            "paper: type({}) -> colon({}) -> space({}) -> desc({})",
            at(self.kind.found_at),
            at(self.colon.found_at),
            at(self.space.found_at),
            at(self.desc.found_at),
        )?;

        write!(f, "|      ")?;
        let mut t: BTreeSet<Pencil> = BTreeSet::new();
        t.insert(self.root);
        t.insert(self.kind);
        t.insert(self.colon);
        t.insert(self.space);
        t.insert(self.desc);
        for one in t {
            if one.subject == Subject::Root {
                continue;
            }
            let pencil = self.find_pencil(one.subject);
            write!(f, "{}({})", one.subject, at(pencil.found_at))?;
            if one.subject != Subject::Desc {
                write!(f, " -> ")?;
            }
        }
        write!(f, "\n")
    }
}
