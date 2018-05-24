pub fn solve(jobs: &Vec<&Job>) -> usize {
    if jobs.len() == 0 {
        return 0;
    }
    let mut max_profit = vec![0usize; jobs.len() + 1];
    for (i, &job) in jobs.iter().enumerate() {
        let profit_prior_jobs = jobs[0..i]
            .iter()
            .enumerate()
            .filter(|&(_, &x)| x.finish <= job.start)
            .map(|(j, _)| max_profit[j + 1] + job.profit)
            .max()
            .unwrap_or(max_profit[i]);

        max_profit[i + 1] = profit_prior_jobs
            .max(job.profit)
            .max(max_profit[i]);
    }
    max_profit
        .last()
        .unwrap()
        .to_owned()
}

pub struct Job {
    start: usize,
    finish: usize,
    profit: usize,
}

#[cfg(test)]
mod tests {
    use super::{Job, solve};

    #[test]
    fn zero_jobs() {
        let jobs: Vec<&Job> = Vec::new();
        let profit = solve(&jobs);
        assert_eq!(profit, 0);
    }

    #[test]
    fn one_job() {
        let jobs: Vec<&Job> = vec![&Job { start: 0, finish: 5, profit: 5 }];
        let profit = solve(&jobs);
        assert_eq!(profit, 5);
    }

    #[test]
    fn simple_test() {
        let jobs: Vec<&Job> = vec![
            &Job { start: 0, finish: 3, profit: 1 },
            &Job { start: 1, finish: 3, profit: 3 }
        ];
        let profit = solve(&jobs);
        assert_eq!(profit, 3);
    }

    #[test]
    fn real_test() {
        let jobs: Vec<&Job> = vec![
            &Job { start: 1, finish: 2, profit: 50 },
            &Job { start: 3, finish: 5, profit: 20 },
            &Job { start: 6, finish: 19, profit: 100 },
            &Job { start: 2, finish: 100, profit: 200 }
        ];
        let profit = solve(&jobs);
        assert_eq!(profit, 250);
    }
}
