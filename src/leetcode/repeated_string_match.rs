///
/// returns the number of times n that a must be repeated so that b is a substring of a*n
///
/// we only have to check the lowest n where len(a*n) > len(b), and then len(
/// # Example
/// ```
/// let a = String::from("abcd");
/// let b = String::from("cdabcdab");
/// let result: usize = rust_practice::leetcode::repeated_string_match::solve(&a, &b).unwrap();
/// assert_eq!(result, 3usize);
/// ```
///
pub fn solve(a: &str, b: &str) -> Option<usize> {
    let num_repeats :usize= (b.len() - 1).wrapping_div(a.len()) + 1;

    let mut a_repeated = a.repeat(num_repeats);
    if a_repeated.contains(b) {
        return Some(num_repeats);
    }
    a_repeated.push_str(a);
    if a_repeated.contains(b) {
        return Some(num_repeats+1);
    }
    None
}

#[cfg(test)]
pub mod tests {
    use super::solve;

    #[test]
    fn test_simple() {
        let a: String = String::from("a");
        let b: String = String::from("aaaa");
        let count: usize = solve(&a, &b).unwrap();
        assert_eq!(count, 4);
    }

    #[test]
    fn test_impossible() {
        let a: String = String::from("b");
        let b: String = String::from("abab");
        let result = solve(&a, &b);
        assert!(result.is_none());
    }
}