#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Compare two lists
pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    return if first_list == second_list {
        Comparison::Equal
    } else if first_list.iter().all(|el| second_list.contains(el)) {
        Comparison::Sublist
    } else if second_list.iter().all(|el| first_list.contains(el)) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
