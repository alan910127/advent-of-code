use crate::prelude::*;

pub fn process(input: &str) -> Result<u64> {
    println!("part2!");
    return Ok(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "";

    #[test]
    fn test_process() -> Result<()> {
        assert_eq!(process(INPUT)?, 0);
        Ok(())
    }
}
