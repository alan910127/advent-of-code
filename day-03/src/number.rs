use crate::engine::{EngineSchematic, EngineSchematicSlot};

pub fn find_numbers(schematic: &EngineSchematic) -> Vec<Number> {
    let mut numbers = Vec::new();
    for (line, slots) in schematic.iter().enumerate() {
        let mut builder = NumberBuilder::new(line);
        for (col, slot) in slots.iter().enumerate() {
            match slot {
                EngineSchematicSlot::Number(d) => builder.push(col, *d),
                _ => {
                    if !builder.is_empty() {
                        numbers.push(builder.build());
                        builder.refresh();
                    }
                }
            }
        }
        if !builder.is_empty() {
            numbers.push(builder.build());
        }
    }
    numbers
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Number {
    pub value: u64,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Span {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl Number {
    pub fn is_part_nubmer(&self, schematic: &[Vec<EngineSchematicSlot>]) -> bool {
        let row_start = self.span.line.saturating_sub(1);
        let row_end = (self.span.line + 1).min(schematic.len() - 1);
        let col_start = self.span.start.saturating_sub(1);
        let col_end = (self.span.end + 1).min(schematic[self.span.line].len() - 1);

        for row in schematic.iter().take(row_end + 1).skip(row_start) {
            for col in row.iter().take(col_end + 1).skip(col_start) {
                if let EngineSchematicSlot::Symbol(_) = col {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NumberBuilder {
    value: u64,
    line: usize,
    start: Option<usize>,
    end: Option<usize>,
}

impl NumberBuilder {
    pub fn new(line: usize) -> Self {
        Self {
            line,
            ..Default::default()
        }
    }

    pub fn push(&mut self, col: usize, digit: u64) {
        if self.start.is_none() {
            self.start = Some(col);
        }
        self.end = Some(col);
        self.value = self.value * 10 + digit;
    }

    pub fn build(self) -> Number {
        Number {
            value: self.value,
            span: Span {
                line: self.line,
                start: self.start.unwrap(),
                end: self.end.unwrap(),
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start.is_none() || self.end.is_none()
    }

    pub fn refresh(&mut self) {
        self.value = 0;
        self.start = None;
        self.end = None;
    }
}
