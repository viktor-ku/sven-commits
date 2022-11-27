use crate::{domain::Domain, pencil::Pencil};
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
                domain: Domain::Root,
                found_at: None,
                next: None,
                prev: None,
            },
            kind: Pencil {
                domain: Domain::Type,
                found_at: None,
                next: None,
                prev: None,
            },
            colon: Pencil {
                domain: Domain::Colon,
                found_at: None,
                next: None,
                prev: None,
            },
            space: Pencil {
                domain: Domain::Space,
                found_at: None,
                next: None,
                prev: None,
            },
            desc: Pencil {
                domain: Domain::Desc,
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

        let mut prev: Option<Domain> = None;
        let mut next: Option<Domain> = None;
        for one in t {
            if let Some(prev_subject) = prev {
                self.find_pencil_mut(one.domain).prev = Some(prev_subject);
            }
            prev = Some(one.domain);

            if let Some(next_subject) = next {
                self.find_pencil_mut(next_subject).next = Some(one.domain);
            }
            next = Some(one.domain);
        }

        debug_assert!(self.root.prev.is_none());
        debug_assert!(self.desc.next.is_none());
    }

    #[inline]
    pub fn find_pencil(&self, domain: Domain) -> &Pencil {
        match domain {
            Domain::Root => &self.root,
            Domain::Type => &self.kind,
            Domain::Colon => &self.colon,
            Domain::Space => &self.space,
            Domain::Desc => &self.desc,
            _ => todo!(),
        }
    }

    #[inline]
    pub fn find_pencil_mut(&mut self, domain: Domain) -> &mut Pencil {
        match domain {
            Domain::Root => &mut self.root,
            Domain::Type => &mut self.kind,
            Domain::Colon => &mut self.colon,
            Domain::Space => &mut self.space,
            Domain::Desc => &mut self.desc,
            _ => todo!(),
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
            if one.domain == Domain::Root {
                continue;
            }
            let pencil = self.find_pencil(one.domain);
            write!(f, "{}({})", one.domain, at(pencil.found_at))?;
            if one.domain != Domain::Desc {
                write!(f, " -> ")?;
            }
        }
        write!(f, "\n")
    }
}
