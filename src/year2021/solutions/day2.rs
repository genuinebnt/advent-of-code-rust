use std::fs;

struct Values {
    depth: i32,
    x_pos: i32,
    aim: i32,
}

fn read_input(path: &str) -> Result<Vec<(String, i32)>, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    let mut tuples: Vec<(String, i32)> = Vec::new();
    for line in contents.lines().filter(|line| !line.is_empty()) {
        let (key, value) = line.split_once(char::is_whitespace).unwrap();
        tuples.push((key.to_string(), value.parse().unwrap()));
    }

    Ok(tuples)
}

fn calc_depth(input: Vec<(String, i32)>) -> i32 {
    let mut values = Values {
        depth: 0,
        x_pos: 0,
        aim: 0,
    };

    for (key, value) in input.into_iter() {
        match &key[..] {
            "forward" => values.x_pos += value,
            "up" => values.depth -= value,
            "down" => values.depth += value,
            _ => unreachable!(),
        }
    }
    values.depth * values.x_pos
}

fn calc_depth_two(input: Vec<(String, i32)>) -> i32 {
    let mut values = Values {
        depth: 0,
        x_pos: 0,
        aim: 0,
    };

    for (key, value) in input.into_iter() {
        match &key[..] {
            "forward" => {
                values.x_pos += value;
                values.depth += values.aim * value;
            }
            "up" => values.aim -= value,
            "down" => values.aim += value,
            _ => unreachable!(),
        }
    }
    values.depth * values.x_pos
}

fn part_one(path: &str) -> i32 {
    let input = read_input(path).unwrap();
    calc_depth(input)
}

fn part_two(path: &str) -> i32 {
    let input = read_input(path).unwrap();
    calc_depth_two(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calc_test() {
        let input: Vec<(String, i32)> = vec![("forward".to_owned(), 5), ("up".to_owned(), 3)];
        assert_eq!(-15, calc_depth(input));
    }
    #[test]
    fn part_one_test() {
        let output = part_one("src/year2021/inputs/day2.txt");
        assert_eq!(2215080, output);
    }

    #[test]
    fn part_two_test() {
        let output = part_two("src/year2021/inputs/day2.txt");
        assert_eq!(1864715580, output);
    }
}
