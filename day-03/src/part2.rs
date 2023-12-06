use std::ops::RangeInclusive;

use crate::{
    engine::{parse_engine, EngineSchematicSlot},
    number::find_numbers,
    prelude::*,
};

trait Overlap {
    fn overlaps_with(&self, other: &Self) -> bool;
}

impl Overlap for RangeInclusive<usize> {
    fn overlaps_with(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

pub fn process(input: &str) -> Result<u64> {
    let schematic = parse_engine(input);
    let numbers = find_numbers(&schematic);

    let possible_gears = schematic.iter().enumerate().flat_map(|(line, slots)| {
        slots
            .iter()
            .enumerate()
            .filter_map(move |(col, slot)| match slot {
                EngineSchematicSlot::Symbol('*') => Some((line, col)),
                _ => None,
            })
    });

    let ratio_sum = possible_gears.fold(0, |acc, gear| {
        let line_range = gear.0.saturating_sub(1)..=(gear.0 + 1).min(schematic.len() - 1);
        let col_range = gear.1.saturating_sub(1)..=(gear.1 + 1).min(schematic[gear.0].len() - 1);

        let (ratio, adjacent_count) = numbers
            .iter()
            .filter(|n| {
                line_range.contains(&n.span.line)
                    && col_range.overlaps_with(&(n.span.start..=n.span.end))
            })
            .fold((1, 0), |(ratio, adjacent_count), n| {
                (ratio * n.value, adjacent_count + 1)
            });

        if adjacent_count == 2 {
            acc + ratio
        } else {
            acc
        }
    });

    Ok(ratio_sum)
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
        assert_eq!(process(INPUT)?, 467835);
        Ok(())
    }
}
