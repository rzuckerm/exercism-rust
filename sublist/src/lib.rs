#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let first_len = first_list.len();
    let second_len = second_list.len();
    let mut result = Comparison::Unequal;
    if first_len == second_len {
        if first_list == second_list {
            result = Comparison::Equal;
        }
    } else if first_len < second_len {
        if rhs_contains_lhs(first_list, second_list) {
            result = Comparison::Sublist;
        }
    } else if rhs_contains_lhs(second_list, first_list) {
        result = Comparison::Superlist;
    }

    result
}

fn rhs_contains_lhs<T: PartialEq>(lhs: &[T], rhs: &[T]) -> bool {
    lhs.is_empty() || rhs.windows(lhs.len()).any(|sublist| sublist == lhs)
}
