#[derive(Copy, Clone)]
pub struct Position<'a> {
    source: &'a str,
    absolute_index: i32,
    column: usize,
    row: usize,
}

impl<'a> Position<'a> {
    pub fn new(source: &'a str) -> Self {
        Position { source, absolute_index: 0, column: 1, row: 1 }
    }

    pub fn advance(&mut self, newline: bool) -> &Position {
        self.column = if newline { 1 } else { self.column + 1 };
        self.row = if newline { self.row + 1 } else { self.row };
        self.absolute_index += 1;

        self
    }
}

impl ToString for Position<'_> {
    fn to_string(&self) -> String {
        let line = self.row - 1;
        let col = self.column - 1;
        format!("\n\n\t\t{}\n\t\t{}^\n\t[Ln:{} Col:{}]", self.source.lines().nth(line).unwrap(), if col > 0 { " ".repeat(col - 1) } else { String::new() }, self.row, self.column)
    }
}