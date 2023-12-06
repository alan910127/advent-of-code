use crate::{engine::parse_engine, number::find_numbers, prelude::*};

pub fn process(input: &str) -> Result<u64> {
    let schematic = parse_engine(input);
    let numbers = find_numbers(&schematic);

    let sum = numbers
        .into_iter()
        .filter_map(|n| {
            if n.is_part_nubmer(schematic.as_slice()) {
                Some(n.value)
            } else {
                None
            }
        })
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_process() -> Result<()> {
        assert_eq!(process(INPUT)?, 4361);
        Ok(())
    }
}
