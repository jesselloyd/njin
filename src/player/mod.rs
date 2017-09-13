use common::game_object::{GameObject, Position, Direction, Moves, Interacts};

#[derive(Clone)]
pub struct Player {
    name: String,
    position: Position,
    speed: i32,
    destroyable: bool,
    blocking: bool,
    facing_direction: Direction,
    range: i32,
}

impl Player {
    pub fn new(name: &str, range: i32, speed: i32, position: Position) -> Player {
        Player {
            name: name.to_string(),
            position: position,
            blocking: false,
            destroyable: true,
            facing_direction: Direction::Forward,
            range: range,
            speed: speed,
        }
    }
}

impl GameObject for Player {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn position(&self) -> Position {
        self.position
    }

    fn is_destroyable(&self) -> bool {
        self.destroyable
    }
}

impl Moves for Player {
    fn movement(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.position.y += self.speed,
            Direction::Down => self.position.y -= self.speed,
            Direction::Forward => self.position.x += self.speed,
            Direction::Backward => self.position.x -= self.speed,
        }
    }
}

impl Interacts for Player {
    fn attack<T: GameObject>(&self, target: &mut T) {
        if self.in_range_of(target) && target.is_destroyable() && target.can_be_attacked() {}
    }

    #[allow(unused)]
    fn interact_with<T: GameObject>(&self, target: &mut T) {
        println!("Interacting with it.");
    }

    fn toggle_block(&mut self) {
        self.blocking = !self.blocking;
    }

    fn is_facing<T: GameObject>(&self, target: &T) -> bool {
        let p = self.position();
        let t_p = target.position();
        match self.facing_direction {
            Direction::Up => t_p.y > p.y,
            Direction::Down => t_p.y < p.y,
            Direction::Forward => t_p.x < p.x,
            Direction::Backward => t_p.x > p.x,
        }
    }

    fn in_range_of<T: GameObject>(&self, target: &T) -> bool {
        let p = self.position();
        let t_p = target.position();
        self.is_facing(target) && (p.x - t_p.x).abs() <= self.range &&
            (p.y - t_p.y).abs() <= self.range
    }

    fn say(&self, phrase: &str) {
        println!("{} says: \"{}\"", self.name, phrase);
    }

    fn focus_on<T: GameObject>(&self, target: &mut T) {
        println!("{} is now focused on {}.", self.name, target.name());
    }

    fn on_death(&mut self) {
        println!("{} died.", self.name);
    }

    fn pick_up<T: GameObject>(&mut self, object: T) {
        println!("Picking up object {}", object.name());
    }

    fn discard<T: GameObject>(&mut self, object: T) -> T {
        object
    }
}
