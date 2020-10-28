use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum PawnType {
    Army,
    Fleet,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
struct Relation<'a> {
    location_a: &'a Location<'a>,
    location_b: &'a Location<'a>,
    pawn_type: PawnType,
}

impl<'a> Relation<'a> {
    pub fn create(location_a: &'a Location<'a>, location_b: &'a Location<'a>, pawn_type: PawnType) -> Relation<'a> {
        Relation { location_a, location_b, pawn_type }
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

    fn add_relation(&mut self, pawn_type: PawnType, source: &str, target: &str) -> Result<(), String> {
        let location_a = match self.locations.get(source) {
            Some(a) => a,
            None => return Err(format!("No such location {}.", source)),
        };
        let location_b = match self.locations.get(target) {
            Some(b) => b,
            None => return Err(format!("No such location {}.", target)),
        };
        let relation = Relation::create(location_a.clone(), location_b.clone(), pawn_type);
        for existing_relation in self.relations.iter() {
            if relation == *existing_relation {
                return Err(format!("The relation {} ←→ {} for type {:?} already exist.", source, target, pawn_type));
            }
        }

        self.relations.push(relation);
        Ok(())
    }

    pub fn add_relations(&mut self, location_name: &str, pawn_type: PawnType, location_dest: &str) -> Result<(), String> {
        let source = match self.get_location(location_name) {
            Some(a) => a,
            None => return Err(format!("No such source location {}.", location_name)),
        };

        let dest = match self.get_location(location_dest) {
            Some(a) => a,
            None => return Err(format!("No such target location {}.", location_name)),
        };

        if source == dest {
            return Err(format!("Source and destination are the same location {}.", location_name));
        }

        Ok(())
    }

    pub fn get_location(&self, location_name: &str) -> Option<&Location<'a>> {
        self.locations.get(location_name)
    }

    pub fn unit_can_move(&self, pawn_type: PawnType, src_location_name: &str, dst_location_name: &str) -> bool {
        true
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
    pub fn test_add_relations() {
        let mut board = Board::create();
        board.add_location(Location::create("par", true, LocationType::Land));
        board.add_location(Location::create("pic", false, LocationType::Coastal));
        board.add_location(Location::create("bre", true, LocationType::Coastal));
        board.add_location(Location::create("man", false, LocationType::Sea));
        board.add_relations("par", PawnType::Army, "pic");
        board.add_relations("par", PawnType::Army, "bre");
        board.add_relations("bre", PawnType::Fleet, "man");
        board.add_relations("man", PawnType::Fleet, "pic");

        assert!(board.unit_can_move(PawnType::Army, "pic", "par"));
        assert!(!board.unit_can_move(PawnType::Fleet, "pic", "par"));
        assert!(board.unit_can_move(PawnType::Fleet, "bre", "man"));
        assert!(!board.unit_can_move(PawnType::Army, "bre", "man"));


    }
}
