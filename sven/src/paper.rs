use crate::{additive::Additive, domain::Domain, pencil::Pencil};
use std::{collections::BTreeSet, fmt::Debug};

#[derive(PartialEq, Eq)]
pub struct Paper {
    pub root: Pencil,
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
                found_at: Some(0),
                next: None,
                prev: None,
                missing: false,
                missing_nth: 1,
                missing_total: 1,
            },
            kind: Pencil {
                domain: Domain::Type,
                found_at: None,
                next: None,
                prev: None,
                missing: true,
                missing_nth: 1,
                missing_total: 1,
            },
            colon: Pencil {
                domain: Domain::Colon,
                found_at: None,
                next: None,
                prev: None,
                missing: true,
                missing_nth: 1,
                missing_total: 1,
            },
            space: Pencil {
                domain: Domain::Space,
                found_at: None,
                next: None,
                prev: None,
                missing: true,
                missing_nth: 1,
                missing_total: 1,
            },
            desc: Pencil {
                domain: Domain::Desc,
                found_at: None,
                next: None,
                prev: None,
                missing: true,
                missing_nth: 1,
                missing_total: 1,
            },
        }
    }

    pub fn build_map(&mut self) {
        {
            let mut t: BTreeSet<Pencil> = BTreeSet::new();
            t.insert(self.root);
            t.insert(self.kind);
            t.insert(self.colon);
            t.insert(self.space);
            t.insert(self.desc);

            let mut prev: Option<Domain> = None;
            let mut next: Option<Domain> = None;
            let mut nth = 1;
            for pencil in t.iter() {
                match pencil.found_at {
                    Some(_) => {
                        nth = 1;
                    }
                    None => {
                        nth += 1;
                        self.find_pencil_mut(pencil.domain).missing_nth = nth;
                        self.find_pencil_mut(pencil.domain).found_at =
                            self.find_pencil(prev.unwrap()).found_at;
                    }
                };

                if let Some(prev_subject) = prev {
                    self.find_pencil_mut(pencil.domain).prev = Some(prev_subject);
                }
                prev = Some(pencil.domain);

                if let Some(next_subject) = next {
                    self.find_pencil_mut(next_subject).next = Some(pencil.domain);
                }
                next = Some(pencil.domain);
            }

            debug_assert!(self.root.prev.is_none());
            debug_assert!(self.desc.next.is_none());
        };

        let mut v: Vec<Domain> = Vec::new();
        let mut next = Some(self.root.domain);
        while let Some(domain) = next {
            v.push(domain);
            next = self.find_pencil(domain).next;
        }
        v.reverse();
        let mut total = 1;
        for domain in v {
            let mut pencil = self.find_pencil_mut(domain);

            if pencil.missing {
                if total == 1 {
                    total = pencil.missing_nth;
                }

                let nth = pencil.missing_nth;
                let base_id = pencil.found_at.unwrap();
                let add = 1_000 / total * (nth - 1);
                pencil.found_at = Some(base_id + add);
                pencil.missing_total = total;
            } else {
                total = 1;
            }
        }
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

        #[inline]
        fn missing<'a>(is: bool) -> &'a str {
            if is {
                "-"
            } else {
                "+"
            }
        }

        writeln!(
            f,
            "paper: {}Type({}) -> {}Colon({}) -> {}Space({}) -> {}Desc({})",
            missing(self.kind.missing),
            at(self.kind.found_at),
            missing(self.colon.missing),
            at(self.colon.found_at),
            missing(self.space.missing),
            at(self.space.found_at),
            missing(self.desc.missing),
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
            write!(
                f,
                "{}{:?}({})",
                missing(one.missing),
                one.domain,
                at(pencil.found_at)
            )?;
            if one.domain != Domain::Desc {
                write!(f, " -> ")?;
            }
        }
        write!(f, "\n")
    }
}
