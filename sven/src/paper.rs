use std::fmt::Debug;

#[derive(PartialEq, Eq)]
pub struct Paper {
    pub type_id: Option<usize>,
    pub colon_id: Option<usize>,
    pub space_id: Option<usize>,
    pub desc_id: Option<usize>,
}

impl Paper {
    pub fn new() -> Self {
        Self {
            type_id: None,
            colon_id: None,
            space_id: None,
            desc_id: None,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == Self {
            type_id: None,
            colon_id: None,
            space_id: None,
            desc_id: None,
        }
    }
}

impl Debug for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "paper: type({}) -> colon({}) -> space({}) -> desc({})",
            match self.type_id {
                Some(id) => format!("{}", id),
                None => "no".to_string(),
            },
            match self.colon_id {
                Some(id) => format!("{}", id),
                None => "no".to_string(),
            },
            match self.space_id {
                Some(id) => format!("{}", id),
                None => "no".to_string(),
            },
            match self.desc_id {
                Some(id) => format!("{}", id),
                None => "no".to_string(),
            }
        )
    }
}
