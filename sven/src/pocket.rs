/// The way to find misplaced tokens as well as navigate between the tokens
/// is by having a virtual collection of Pockets. Having these in e.g.
/// a fixed vector with already known capacity (because we know how
/// many tokens there are in the conventional commit) where every item has
/// its own space and can be pointed to or by another pocket is crucial when
/// building an idea of where everything is and where it should go by e.g.
/// comparing two sets of Pockets (expected, with tokens in the right order, and
/// an actual, with the tokens derived from the input)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pocket {
    Empty,
    Missing,
    Token(usize),
}

impl Pocket {
    /// New pocket always corresponds to the empty, essentially just reserving
    /// the space, which is very important for later navigation between nodes
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_missing(&self) -> bool {
        match self {
            Self::Missing => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_token(&self) -> bool {
        match self {
            Self::Token(_) => true,
            _ => false,
        }
    }

    /// Extracts an id from the token, assuming it is token
    #[inline]
    pub fn id(&self) -> Option<usize> {
        match self {
            Self::Token(id) => Some(*id),
            _ => None,
        }
    }
}

impl Default for Pocket {
    fn default() -> Self {
        Self::Empty
    }
}
