use day_01::{part2, prelude::*};

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("../../input.txt");
    let answer = part2::process(input);
    println!("answer = {:?}", answer);

    Ok(())
}
