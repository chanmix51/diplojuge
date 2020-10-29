use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
enum PawnType {
    Army,
    Fleet,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Player {
    GB,
    FR,
    GE,
    IT,
    AH,
    RU,
    TU,
}

#[derive(Debug, PartialEq)]
enum LocationType {
    Land,
    Coastal,
    Sea,
}

#[derive(Debug, PartialEq)]
struct Pawn {
    unit: PawnType,
    player: Player,
}

#[derive(Debug, PartialEq)]
struct Location<'a> {
    name: &'a str,
    is_center: bool,
    location_type: LocationType,
    owned_by: Option<Player>
}

impl<'a> Location<'a> {
    pub fn create(name: &'a str, is_center: bool, location_type: LocationType) -> Location<'a> {
        Location {
            name,
            is_center,
            location_type,
            owned_by: None,
        }
    }

    pub fn is_owned_by(&mut self, player: Player) -> Result<(), String> {
        if !self.is_center {
            return Err(format!("Cannot assign location {} to player {:?}, it is not a center.", self.name, player));
        }
        self.owned_by = Some(player);

        Ok(())
    }
}

#[derive(Debug)]
struct Relation<'a> {
    location_a: &'a str,
    location_b: &'a str,
    pawn_type: PawnType,
}

impl<'a> Relation<'a> {
    pub fn create(location_a: &'a str, location_b: &'a str, pawn_type: PawnType) -> Relation<'a> {
        Relation { location_a, location_b, pawn_type }
    }
}

impl<'a> PartialEq for Relation<'a> {
    fn eq(&self, other: &Relation) -> bool {
        self.pawn_type == other.pawn_type && (
            self.location_a == other.location_a
            && self.location_b == other.location_b 
            || self.location_a == other.location_b
            && self.location_b == other.location_a
            )
    }
}

#[derive(Debug)]
struct Board<'a> {
    locations: HashMap<&'a str, Location<'a>>,
    relations: Vec<Relation<'a>>,
}

impl<'a> Board<'a> {
    pub fn create() -> Board<'a> {
        Board { locations: HashMap::new(), relations: Vec::new() }
    }

    pub fn add_location(&mut self, location: Location<'a>) {
        self.locations.insert(location.name, location);
    }

    fn location_exists(&self, location: &str) -> bool {
        match self.locations.get(location) {
            Some(_) => true,
            None => false,
        }
    }

    fn check_location(&self, location: &str) -> Result<(), String> {
        if self.location_exists(location) {
            Ok(())
        } else {
            Err(format!("Location {} does not exist.", location))
        }
    }

    fn relation_exists(&self, relation: &Relation) -> bool {
        for existing_relation in self.relations.iter() {
            if *relation == *existing_relation {
                return true;
            }
        }

        false
    }

    fn add_relation(&mut self, pawn_type: PawnType, source: &'a str, target: &'a str) -> Result<(), String> {
        self.check_location(source)?;
        self.check_location(target)?;
        if source == target {
            return Err(format!("Trying to define a self relation {} ←→ {}", source, target));
        }

        let relation = Relation::create(source, target, pawn_type);
        if self.relation_exists(&relation) {
                return Err(format!("The relation {} ←→ {} already exist.", source, target));
        }

        self.relations.push(relation);
        Ok(())
    }

    pub fn get_location(&self, location_name: &str) -> Option<&Location<'a>> {
        self.locations.get(location_name)
    }

    pub fn get_location_mut(&mut self, location_name: &str) -> Option<&mut Location<'a>> {
        self.locations.get_mut(location_name)
    }

    pub fn unit_can_move(&self, pawn_type: PawnType, src_location_name: &str, dst_location_name: &str) -> bool {
        let my_relation = Relation::create(src_location_name, dst_location_name, pawn_type);
        self.relation_exists(&my_relation)
    }

    pub fn assign_center(&mut self, location_name: &str, player: Player) -> Result<(), String> {
        match self.get_location_mut(location_name) {
            Some(location) => location.is_owned_by(player),
            None => Err(format!("No such location {}.", location_name))
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn test_set_board() {
        let mut board = Board::create();
        let location = Location::create("bre", true, LocationType::Coastal);
        let bw_location = &location;
        board.add_location(location);

        assert_eq!("bre", board.get_location("bre").unwrap().name);
    }

    #[test]
    pub fn test_add_relation() {
        let mut board = Board::create();
        board.add_location(Location::create("par", true, LocationType::Land));
        board.add_location(Location::create("pic", false, LocationType::Coastal));
        board.add_location(Location::create("bre", true, LocationType::Coastal));
        board.add_location(Location::create("man", false, LocationType::Sea));
        board.add_relation(PawnType::Army, "par", "pic");
        board.add_relation(PawnType::Army, "par", "bre");
        board.add_relation(PawnType::Fleet, "bre", "man");
        board.add_relation(PawnType::Fleet, "man", "pic");

        assert!(board.unit_can_move(PawnType::Army, "pic", "par"));
        assert!(!board.unit_can_move(PawnType::Fleet, "pic", "par"));
        assert!(board.unit_can_move(PawnType::Fleet, "bre", "man"));
        assert!(!board.unit_can_move(PawnType::Army, "bre", "man"));
        assert!(!board.unit_can_move(PawnType::Army, "abc", "man"));
    }

    #[test]
    pub fn test_assign_location_success() {
        let mut board = Board::create();
        board.add_location(Location::create("par", true, LocationType::Land));
        board.assign_center("par", Player::FR);
        assert!(true);
    }
}
