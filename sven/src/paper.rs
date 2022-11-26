use crate::{pencil::Pencil, subject::Subject};
use std::{collections::BTreeSet, fmt::Debug};

#[derive(Debug, PartialEq, Eq)]
pub struct Paper {
    root: Pencil,
    pub kind: Pencil,
    pub colon: Pencil,
    pub space: Pencil,
    pub desc: Pencil,
}

impl Paper {
    const ID_ROOT: usize = 0;
    const ID_KIND: usize = 1;
    const ID_COLON: usize = 2;
    const ID_SPACE: usize = 3;
    const ID_DESC: usize = 4;

    #[inline]
    pub fn new() -> Self {
        Self {
            root: Pencil {
                id: Self::ID_ROOT,
                subject: Subject::Root,
                found_at: None,
                next: None,
                prev: None,
            },
            kind: Pencil {
                id: Self::ID_KIND,
                subject: Subject::Kind,
                found_at: None,
                next: None,
                prev: None,
            },
            colon: Pencil {
                id: Self::ID_COLON,
                subject: Subject::Colon,
                found_at: None,
                next: None,
                prev: None,
            },
            space: Pencil {
                id: Self::ID_SPACE,
                subject: Subject::Space,
                found_at: None,
                next: None,
                prev: None,
            },
            desc: Pencil {
                id: Self::ID_DESC,
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
    pub fn find_pencil_by_id(&self, id: usize) -> &Pencil {
        match id {
            Self::ID_ROOT => &self.root,
            Self::ID_KIND => &self.kind,
            Self::ID_COLON => &self.colon,
            Self::ID_SPACE => &self.space,
            Self::ID_DESC => &self.desc,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn find_pencil_by_id_mut(&mut self, id: usize) -> &mut Pencil {
        match id {
            Self::ID_ROOT => &mut self.root,
            Self::ID_KIND => &mut self.kind,
            Self::ID_COLON => &mut self.colon,
            Self::ID_SPACE => &mut self.space,
            Self::ID_DESC => &mut self.desc,
            _ => unreachable!(),
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
