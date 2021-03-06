use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum PawnType {
    Army,
    Fleet,
}

#[derive(Debug, PartialEq)]
pub enum LocationType {
    Land,
    Coastal,
    Sea,
}

#[derive(Debug, PartialEq)]
pub struct Location<'a> {
    name: &'a str,
    is_center: bool,
    location_type: LocationType,
}

impl<'a> Location<'a> {
    pub fn create(name: &'a str, is_center: bool, location_type: LocationType) -> Location<'a> {
        Location {
            name,
            is_center,
            location_type
        }
    }
}

#[derive(Debug)]
pub struct Relation<'a> {
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
pub struct Board<'a> {
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

    pub fn has_location(&self, location: &'a str) -> bool {
        self.locations.get(location).is_some()
    }

    pub fn check_location(&self, location: &str) -> Result<(), String> {
        if self.has_location(location) {
            Ok(())
        } else {
            Err(format!("Location {} does not exist.", location))
        }
    }

    pub fn relation_exists(&self, relation: &Relation) -> bool {
        for existing_relation in self.relations.iter() {
            if *relation == *existing_relation {
                return true;
            }
        }

        false
    }

    pub fn add_relation(&mut self, pawn_type: PawnType, source: &'a str, target: &'a str) -> Result<(), String> {
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

    pub fn unit_can_move(&self, pawn_type: PawnType, src_location_name: &str, dst_location_name: &str) -> bool {
        let my_relation = Relation::create(src_location_name, dst_location_name, pawn_type);
        self.relation_exists(&my_relation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn test_set_board() {
        let mut board = Board::create();
        let location = Location::create("bre", true, LocationType::Coastal);
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
        board.add_relation(PawnType::Army, "par", "pic").unwrap();
        board.add_relation(PawnType::Army, "par", "bre").unwrap();
        board.add_relation(PawnType::Fleet, "bre", "man").unwrap();
        board.add_relation(PawnType::Fleet, "man", "pic").unwrap();

        assert!(board.unit_can_move(PawnType::Army, "pic", "par"));
        assert!(!board.unit_can_move(PawnType::Fleet, "pic", "par"));
        assert!(board.unit_can_move(PawnType::Fleet, "bre", "man"));
        assert!(!board.unit_can_move(PawnType::Army, "bre", "man"));
        assert!(!board.unit_can_move(PawnType::Army, "abc", "man"));
    }
}
