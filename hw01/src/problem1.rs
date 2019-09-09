use std::collections::HashSet;

/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut dup = HashSet::new();

    for i in vs {
        if dup.contains(i) {
            continue;
        }
        // https://doc.rust-lang.org/book/second-edition/ch08-03-hash-maps.html#hash-maps-and-ownership
        dup.insert(*i);
        result.push(*i);
    }
    result
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in vs {
        if pred(*i) {
            result.push(*i)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // these are provided tests
    #[test]
    fn test_sum_small() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(sum(&array), 15);
    }

    // Part 2

    #[test]
    fn test_dedup_small() {
        let vs = vec![1, 2, 2, 3, 4, 1];
        assert_eq!(dedup(&vs), vec![1, 2, 3, 4]);
    }

    // Part 3

    fn even_predicate(x: i32) -> bool {
        (x % 2) == 0
    }

    #[test]
    fn test_filter_small() {
        let vs = vec![1, 2, 3, 4, 5];
        assert_eq!(filter(&vs, &even_predicate), vec![2, 4]);
    }

    // my own tests
    #[test]
    fn test_sum_one_element() {
        let array = [1];
        assert_eq!(sum(&array), 1);
    }

    #[test]
    fn test_dedup_no_dup() {
        let vs = vec![1, 2, 3, 4];
        assert_eq!(dedup(&vs), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_filtering_bigger_than_5() {
        let vs = vec![2, 4, 5, 6];
        // awesome! lambda~
        assert_eq!(filter(&vs, &|i: i32| -> bool { i < 5 }), vec![2, 4]);
    }
}