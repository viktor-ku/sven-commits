/// It all seem to boil down to these two types of issues:
/// - expected set of characters
/// - unexpected set of characters
///
/// For examples:
/// - unexpected space(s) at {col}
/// - unexpected "!" at {col}
/// - expected {type} at {col}
/// - expected description at {col} (after ": "?)
/// - expected ")" at {col} (after "(myscope"?)
/// - expected ":" at {col}
///     * when we have at least two words, we would like to consider
///     the first word to be the type, so we take an optimistic approach
///     and ask for expected ":"
#[derive(Debug, PartialEq, Eq)]
pub enum TypeIssue {
    NotFound,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Issue {
    Type(TypeIssue),
}
