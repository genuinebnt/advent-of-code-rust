use std::fs;

fn count_depth_increase(depths: Vec<i32>) -> usize {
    depths.windows(2).filter(|x| x[0] < x[1]).count()
}

fn part_one(path: &str) -> Result<usize, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    let depths = contents
        .lines()
        .map(|value| value.parse().unwrap())
        .collect::<Vec<i32>>();
    Ok(count_depth_increase(depths))
}

fn part_two(path: &str) -> Result<usize, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    let depths = contents
        .lines()
        .map(|value| value.parse().unwrap())
        .collect::<Vec<i32>>();

    let window_sum = depths
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>();

    Ok(window_sum.windows(2).filter(|x| x[0] < x[1]).count())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn count_depth_increase_test() {
        let depths = vec![10, 20, 30, 10, 40, 60, 1, 7];
        let count = count_depth_increase(depths);
        assert_eq!(5, count);
    }

    #[test]
    fn solution_test() {
        let file = "src/year2021/inputs/day1.txt";
        assert_eq!(1292, part_one(file).unwrap())
    }

    #[test]
    fn solution_2_test() {
        let file = "src/year2021/inputs/day1.txt";
        assert_eq!(1262, part_two(file).unwrap())
    }
}
