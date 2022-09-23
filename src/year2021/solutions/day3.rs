use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn find_msb(contents: &str, i: usize) -> char {
    let count_one = contents
        .lines()
        .map(|v| v.chars().nth(i).unwrap())
        .filter(|&v| v == '1')
        .count();
    let count_zero = contents.lines().count() - count_one;
    if count_one > count_zero {
        '1'
    } else {
        '0'
    }
}

fn part_one(path: &str) -> isize {
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    let contents = read_file(path);
    for i in 0..contents.lines().next().unwrap().len() {
        let value = find_msb(&contents, i);
        if value == '1' {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    println!("{}", isize::from_str_radix(&gamma, 2).unwrap());

    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_msb_test() {
        assert_eq!('0', find_msb(&read_file("src/year2021/inputs/day3.txt"), 2));
    }

    #[test]
    fn part_one_test() {
        assert_eq!(1307354, part_one("src/year2021/inputs/day3.txt"))
    }
}
