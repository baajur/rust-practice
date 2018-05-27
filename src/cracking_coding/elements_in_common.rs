///
/// takes two sorted arrays and returns the number of elements they have in common
/// because they are sorted this can be O(n)
///
pub fn solve(x: &[usize], y: &[usize]) -> usize {
    let mut x = x.iter().peekable();
    let mut y = y.iter().peekable();
    let mut count: usize = 0;
    loop {
        match (x.peek(), y.peek()) {
            (Some(&x_val), Some(&y_val)) if x_val == y_val => {
                count += 1;
                x.next();
                y.next();
            }
            (Some(&x_val), Some(&y_val)) if x_val < y_val => {
                x.next();
            }
            (Some(&x_val), Some(&y_val)) if x_val > y_val => {
                y.next();
            }
            _ => break
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_simple() {
        let x: Vec<usize> = vec![0, 4, 5, 6];
        let y: Vec<usize> = vec![1, 2, 3, 5];
        let result = solve(&x, &y);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_zero() {
        let result = solve(&Vec::new(), &Vec::new());
        assert_eq!(result, 0);
        let x: Vec<usize> = vec![0, 2, 4, 6];
        let y: Vec<usize> = vec![1];
        let result = solve(&x, &y);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_complex() {
        let x: Vec<usize> = vec![0, 1, 4, 7, 8, 9, 10, 42];
        let y: Vec<usize> = vec![4, 42, 57, 90];
        let result = solve(&x, &y);
        assert_eq!(result, 2);
    }
}