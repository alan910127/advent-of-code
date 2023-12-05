use crate::prelude::*;

pub fn process(input: &str) -> Result<u64> {
    let sum = input
        .lines()
        .map(process_line)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum();

    Ok(sum)
}

fn process_line(line: &str) -> Result<u64> {
    let first = find_first_digit(line).expect("no first digit found");
    let last = find_last_digit(line).expect("no last digit found");
    Ok(first * 10 + last)
}

macro_rules! ends_with_then_return {
    ($line:expr, $($suffix:expr => $value:expr),*, $(,)?) => {
        $(if $line.ends_with($suffix) {
            return Some($value);
        })*
    };
}

fn find_first_digit(line: &str) -> Option<u64> {
    let mut chars = line.chars();
    for size in 1..=line.len() {
        if let Some(digit) = chars.next().and_then(|c| c.to_digit(10)) {
            return Some(digit as u64);
        }
        let sliced = &line[..size];
        ends_with_then_return! {
            sliced,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
        };
    }
    None
}

macro_rules! starts_with_then_return {
    ($line:expr, $($prefix:expr => $value:expr),*, $(,)?) => {
        $(if $line.starts_with($prefix) {
            return Some($value);
        })*
    };
}

fn find_last_digit(line: &str) -> Option<u64> {
    let mut chars = line.chars().rev();
    for size in 1..=line.len() {
        if let Some(digit) = chars.next().and_then(|c| c.to_digit(10)) {
            return Some(digit as u64);
        }
        let sliced = &line[line.len() - size..];
        starts_with_then_return! {
            sliced,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const INPUT: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_process() -> Result<()> {
        assert_eq!(process(INPUT)?, 281);
        Ok(())
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    /// Digits that are overlapped still count
    #[case("twone", 21)]
    fn test_parse_line(#[case] line: &str, #[case] expected: u64) -> Result<()> {
        assert_eq!(process_line(line)?, expected);
        Ok(())
    }
}
