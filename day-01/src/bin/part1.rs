use day_01::{part1, prelude::*};

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("../../input.txt");
    let answer = part1::process(input);
    println!("answer = {:?}", answer);

    Ok(())
}
