pub fn solve(stocks: Vec<f32>) -> Result<f32, String> {
    if stocks.len() == 0 || stocks.len() == 1 {
        return Ok(0f32);
    }
    let mut result: Vec<f32> = vec![0f32; stocks.len() + 1];
    let mut min: f32 = stocks[0];

    for i in 2..result.len() {
        min = min.min(stocks[i - 1]);
        result[i] = result[i - 1].max(stocks[i - 1] - min);
    }
    result
        .last()
        .map(|x| *x)
        .ok_or("error lol".to_owned())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn zero_stocks() {
        let result = solve(Vec::new());
        assert_eq!(result.unwrap(), 0f32);
    }

    #[test]
    fn simple_case() {
        let result = solve(vec![0f32, 1f32, 2f32, 3f32, 4f32]);
        assert_eq!(result.unwrap(), 4f32);
    }

    #[test]
    fn another_simple_case() {
        let result = solve(vec![7f32, 1f32, 5f32, 3f32, 6f32, 4f32]);
        assert_eq!(result.unwrap(), 5f32);
    }

    #[test]
    fn impossible_case() {
        let result = solve(vec![5f32, 4f32, 3f32]);
        assert_eq!(result.unwrap(), 0f32);
    }
}