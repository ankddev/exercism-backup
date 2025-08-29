#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Compare two lists
pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|list| list == second_list)
    {
        Comparison::Superlist
    } else if first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|list| list == first_list)
    {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
