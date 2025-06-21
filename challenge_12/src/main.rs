#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    } else if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    } else if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}

fn is_sublist(sub: &[i32], full: &[i32]) -> bool {
    if sub.is_empty() {
        return true;
    }
    if sub.len() > full.len() {
        return false;
    }

    full.windows(sub.len()).any(|window| window == sub)
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3], vec![1, 2, 3], Comparison::Equal),
        (vec![1, 2, 3], vec![], Comparison::Superlist),
        (vec![], vec![1, 2, 3], Comparison::Sublist),
        (vec![3, 4], vec![1, 2, 3, 4, 5], Comparison::Sublist),
        (vec![1, 2, 4], vec![1, 2, 3, 4, 5], Comparison::Unequal),
        (vec![1, 3, 2], vec![1, 2, 3], Comparison::Unequal),
        (vec![1, 2, 3, 4, 5], vec![2, 3, 4], Comparison::Superlist),
    ];

    for (a, b, expected) in tests {
        let result = sublist(&a, &b);
        println!("A: {:?}, B: {:?} => Result: {:?} [{}]", a, b, result, result == expected);
    }
}
