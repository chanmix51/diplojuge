use std::collections::HashMap;
use crate::board::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    GB,
    FR,
    GE,
    IT,
    AH,
    RU,
    TU,
}

#[derive(Debug, PartialEq)]
pub struct Pawn {
    unit: PawnType,
    player: Player,
}

pub struct Adjucator<'a> {
    board: Board<'a>,
    pawns: HashMap<&'a str, Pawn>,
    centers: HashMap<&'a str, Player>,
}

impl<'a> Adjucator<'a> {
    pub fn create(board: Board<'a>) -> Self {
        Self {
            board,
            pawns: HashMap::new(),
            centers: HashMap::new(),
        }
    }

    pub fn place_pawn(&mut self, location: &'a str, pawn: Pawn) -> Result<(), String> {
        let _ = self.board.check_location(location)?;
        if let Some(somebody) = self.pawns.get(location) {
            return Err(format!("Location {} already hosts a pawn {:?}.", location, somebody));
        }

        self.pawns.insert(location, pawn);

        Ok(())
    }

    pub fn has_pawn(self, location: &'a str) -> bool {
        self.pawns.get(location).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}