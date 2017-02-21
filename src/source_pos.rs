use std::fmt;


/// The line and column position.
#[derive(Debug, Clone, PartialEq)]
pub struct SourcePos(u32, u32);

impl SourcePos {
    pub fn new(line: u32, col: u32) -> Self {
        SourcePos(line, col)
    }
}

impl fmt::Display for SourcePos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line: {:2} col: {:2}", self.0, self.1)
    }
}
