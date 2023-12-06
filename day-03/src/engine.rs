pub type EngineSchematic = Vec<Vec<EngineSchematicSlot>>;

pub fn parse_engine(schematic: &str) -> EngineSchematic {
    schematic.lines().map(parse_line).collect::<Vec<_>>()
}

fn parse_line(line: &str) -> Vec<EngineSchematicSlot> {
    line.chars()
        .map(|c| {
            if let Some(d) = c.to_digit(10) {
                EngineSchematicSlot::Number(d as u64)
            } else if c == '.' {
                EngineSchematicSlot::Empty
            } else {
                EngineSchematicSlot::Symbol(c)
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub enum EngineSchematicSlot {
    Empty,
    Symbol(char),
    Number(u64),
}
