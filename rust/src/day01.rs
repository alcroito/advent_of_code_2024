use itertools::Itertools;

fn main() {
    color_eyre::install().expect("Error setting up error handler");
    let d1_input = read_file_contents_to_string("inputs/day01.txt");
    let res = part1(&d1_input);
    println!("Part 1: {}", res);

    let res = part2(&d1_input);
    println!("Part 2: {}", res);
}

fn read_file_contents_to_string(path: &str) -> String {
    let path = [env!("CARGO_MANIFEST_DIR"), "..", path]
        .iter()
        .collect::<std::path::PathBuf>();
    std::fs::read_to_string(path).expect("Error reading file")
}

// Compute the sum of differences between lowest number on left and right side,
// second lowest number on left and right side, etc.
fn part1(s: &str) -> i64 {
    let (mut v1, mut v2) = s
        .lines()
        .map(|line| {
            let nums: (i64, i64) = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().expect("invalid number"))
                .collect_tuple()
                .expect("Invalid two numbers");
            nums
        })
        .fold((vec![], vec![]), |(mut left, mut right), (a, b)| {
            left.push(a);
            right.push(b);
            (left, right)
        });
    v1.sort();
    v2.sort();
    let sum = v1.iter().zip(v2.iter()).map(|(a, b)| (b - a).abs()).sum();
    sum
}

// Multiple the numbers in the left side with the count they appear in the right side.
fn part2(s: &str) -> i64 {
    let (v1, v2) = s
        .lines()
        .map(|line| {
            let nums: (i64, i64) = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().expect("invalid number"))
                .collect_tuple()
                .expect("Invalid two numbers");
            nums
        })
        .fold((vec![], vec![]), |(mut left, mut right), (a, b)| {
            left.push(a);
            right.push(b);
            (left, right)
        });
    let counts = v2.iter().counts();
    v1.iter()
        .map(|i| i64::try_from(*counts.get(i).unwrap_or(&0)).expect("Too big count") * i)
        .sum()
}

// Write a test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1() {
        color_eyre::install().expect("Error setting up error handler");

        let s = "
3   4
4   3
2   5
1   3
3   9
3   3
        ";
        let s = s.trim();
        let res = part1(s);
        assert_eq!(res, 11);

        let res = part2(s);
        assert_eq!(res, 31);
    }
}
