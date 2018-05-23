fn merge(result: &mut Vec<usize>, lower: usize, mid: usize, upper: usize) {
    let mut out: Vec<usize> = Vec::new();
    {
        let mut left = result[lower..(mid + 1)].iter().peekable();
        let mut right = result[(mid + 1)..(upper + 1)].iter().peekable();
        while let (Some(&l), Some(&r)) = (left.peek(), right.peek()) {
            if *l <= *r {
                out.push(*(left.next().unwrap()));
            } else {
                out.push(*(right.next().unwrap()));
            }
        }
        for x in left {
            out.push(*x);
        }
        for y in right {
            out.push(*y);
        }
    }
    result[lower..(upper + 1)].clone_from_slice(&out[..]);
}

fn recurse(x: &Vec<usize>, lower: usize, upper: usize, result: &mut Vec<usize>) {
    if lower >= upper {
        result[lower] = x[lower];
    } else {
        let mid = lower + (upper - lower) / 2;
        recurse(x, lower, mid, result);
        recurse(x, mid + 1, upper, result);
        merge(result, lower, mid, upper);
    }
}

pub fn solve(x: &Vec<usize>) -> Vec<usize> {
    let len = x.len();
    if len < 2 {
        return x.clone();
    }
    let mut result: Vec<usize> = vec![0; len];
    recurse(x, 0, len-1, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::{merge, solve};

    #[test]
    fn zero_items() {
        let x: Vec<usize> = Vec::new();
        let result = solve(&x);
        assert_eq!(result, Vec::new());
    }

    #[test]
    fn merge_simple() {
        let mut x: Vec<usize> = vec![0, 3, 2, 4];
        merge(&mut x, 0, 1, 3);
        assert_eq!(x, vec![0, 2, 3, 4]);
    }

    #[test]
    fn sort_one_item() {
        let x: Vec<usize> = vec![1];
        let result = solve(&x);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn sort_two_items() {
        let x: Vec<usize> = vec![2, 1];
        let result = solve(&x);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn simple_case() {
        let x: Vec<usize> = vec![5, 3, 9, 8];
        let result = solve(&x);
        assert_eq!(result, vec![3, 5, 8, 9]);
    }
}
