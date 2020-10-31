extern crate diplojuge;
use diplojuge::board::*;
use diplojuge::adjucator::*;

fn configure_board<'a>() -> Board<'a> {
    let mut board = Board::create();
    board.add_location(Location::create("par", true, LocationType::Land));
    board.add_location(Location::create("pic", false, LocationType::Coastal));
    board.add_location(Location::create("bre", true, LocationType::Coastal));
    board.add_location(Location::create("man", false, LocationType::Sea));
    board.add_relation(PawnType::Army, "par", "pic").unwrap();
    board.add_relation(PawnType::Army, "par", "bre").unwrap();
    board.add_relation(PawnType::Fleet, "bre", "man").unwrap();
    board.add_relation(PawnType::Fleet, "man", "pic").unwrap();
    
    board
}

#[test]
fn testcase1() {
    let board = configure_board();
    let adjucator = {
        let mut adjucator = Adjucator::create(board);
        adjucator.place_pawn("par", Pawn { unit: PawnType::Army, player: Player::FR });

        adjucator
    };
    assert!(adjucator.has_pawn("par"));
    assert!(!adjucator.has_pawn("bre"))
}