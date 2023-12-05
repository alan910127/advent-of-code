use crate::prelude::*;

pub fn process(input: &str) -> Result<u64> {
    input
        .lines()
        .map(process_line)
        .collect::<Result<Vec<_>>>()
        .map(|v| v.into_iter().sum::<u64>())
}

fn process_line(line: &str) -> Result<u64> {
    let first = line
        .matches(|c: char| c.is_ascii_digit())
        .next()
        .ok_or_else(|| Error::InvalidInput("no first digit found".to_string()))?
        .parse::<u64>()?;

    let last = line
        .rmatches(|c: char| c.is_ascii_digit())
        .next()
        .ok_or_else(|| Error::InvalidInput("no last digit found".to_string()))?
        .parse::<u64>()?;

    Ok(first * 10 + last)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_process() -> Result<()> {
        assert_eq!(process(INPUT)?, 142);
        Ok(())
    }
}
