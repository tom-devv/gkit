use crate::error::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum Page {
    #[default]
    Overview = 0, // Numbering is for easy recogonition of order on the top bar
    Cadence = 1,
    Todo = 2,
}

pub fn nav() -> Result<()> {
    todo!("")
}
