///Calculates factorial for given number.
/// # Panics
/// Panics if the number is bigger than 20, as `usize` would simply overflow
pub fn factorial(n: usize) -> usize {
    assert!(n <= 20, "n! is to big to calculate");

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

///generates permutation from given `set` and `idx`, then writes it back into set
/// # Panics
/// Panics if idx is more than the number of permutations
pub fn permutation_from_idx<T>(set: &mut [T], idx: usize) {
    let len = set.len();
    let len_factorial = factorial(len);

    assert!(idx < len_factorial, "idx is out of range");

    match len {
        0 | 1 => {}
        _ => {
            let slice_len_fact = factorial(len - 1);
            let local_idx = idx / slice_len_fact;

            for i in 1..=local_idx {
                set.swap(0, i);
            }

            permutation_from_idx(&mut set[1..], idx % slice_len_fact);
        }
    }
}

///Returns `true` if idx of `a` is less or equal to idx of `b`. Panics if neither of the elements are found in set.
fn compare_members<T: PartialEq>(set: &[T], a: &T, b: &T) -> bool {
    for x in set {
        if x == a {
            return true;
        }
        if x == b {
            return false;
        }
    }
    panic!("'a' and 'b' are not members of the set");
}

///From given `set` and `permutation` attempts to find it's index.
/// # Panics
/// * if at least one of the elements from `permutation` is not within `set`
/// * if length of `permutation` is more than 20 (as `usize`) would overflow
pub fn idx_from_permutation<T: PartialEq>(set: &[T], permutation: &[T]) -> usize {
    assert!(
        set.len() >= permutation.len(),
        "Set is smaller than permutation"
    );
    assert!(
        permutation.iter().all(|el| { set.contains(el) }),
        "Not all elements can be found within set"
    );

    let len = permutation.len();

    match len {
        0 | 1 => 0,
        _ => {
            let mut count = 0;
            for x in &permutation[1..] {
                if compare_members(set, x, &permutation[0]) {
                    count += 1;
                }
            }
            let result = count * factorial(len - 1) + idx_from_permutation(set, &permutation[1..]);
            result
        }
    }
}

///Returns a vector of all permutations from given `set`
pub fn permutations<T: Clone>(set: &[T]) -> Vec<Vec<T>> {
    let result_count = factorial(set.len());
    let mut result = Vec::new();

    for i in 0..result_count {
        let mut perm = Vec::from(set);
        permutation_from_idx(&mut perm, i);
        result.push(perm);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::*;
    use permutations::*;

    #[test]
    ///Checks if generated permutation are properly ordered.
    fn permutations_are_ordered() {
        let set = [0, 1, 2, 3, 4];

        let result = permutations(&set);
        for arr in &result {
            println!("{arr:?}");
        }
        let is_sorted = result.is_sorted_by(|a, b| {
            for (a, b) in a.iter().zip(b) {
                if a < b {
                    return true;
                }
            }
            return false;
        });
        assert!(is_sorted);
    }

    #[test]
    ///Checks if outputs of `permutation_from_idx` and `idx_from_permutation` works properly with each other
    fn permutation_idx_correct() {
        let set = [0, 1, 2, 3, 4];

        let perm_count = factorial(set.len());
        for i in 0..perm_count {
            let mut permutation = Vec::from(set);
            permutation_from_idx(&mut permutation, i);
            assert_eq!(idx_from_permutation(&set, &permutation), i);
        }
    }
}
