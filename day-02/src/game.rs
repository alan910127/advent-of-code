use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    multi::separated_list0, IResult,
};

#[derive(Debug)]
pub struct Game {
    pub id: u64,
    pub bags: Vec<Bag>,
}

pub fn process_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, bags) = separated_list0(tag("; "), process_bag)(input)?;

    Ok((
        input,
        Game {
            id: id.parse().unwrap(),
            bags,
        },
    ))
}

#[derive(Debug, Default)]
pub struct Bag {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

impl Bag {
    pub fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn process_bag(input: &str) -> IResult<&str, Bag> {
    let mut bag = Bag::default();

    let (input, _) = separated_list0(
        tag(", "),
        alt((
            map(process_color("red"), |n| bag.red += n),
            map(process_color("green"), |n| bag.green += n),
            map(process_color("blue"), |n| bag.blue += n),
        )),
    )(input)?;

    Ok((input, bag))
}

fn process_color<'a>(color: &'static str) -> impl Fn(&'a str) -> IResult<&'a str, u64> {
    move |input| {
        let (input, number) = digit1(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, _) = tag(color)(input)?;

        Ok((input, number.parse().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn test_process_game(#[case] input: &str, #[case] expected: bool) {
        let (_, game) = process_game(input).unwrap();
        assert_eq!(game.bags.iter().all(Bag::is_valid), expected);
    }
}
