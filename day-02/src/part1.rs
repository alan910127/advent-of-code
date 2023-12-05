use crate::{
    game::{process_game, Bag},
    prelude::*,
};

pub fn process(input: &str) -> Result<u64> {
    let games = input
        .lines()
        .map(|line| {
            process_game(line).map_or_else(|_| Err(Error::ParseError), |(_, game)| Ok(game))
        })
        .collect::<Result<Vec<_>>>()?;

    let sum = games
        .into_iter()
        .filter_map(|game| game.bags.iter().all(Bag::is_valid).then_some(game.id))
        .sum::<u64>();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_process() -> Result<()> {
        assert_eq!(process(INPUT)?, 8);
        Ok(())
    }
}
