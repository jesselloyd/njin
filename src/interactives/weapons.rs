use common::{GameObject, Position};

pub enum WeaponType {
    Sword,
    Axe,
    Polearm,
    Knives,
    Club,
    Staff,
}

#[allow(unused)]
pub struct Weapon {
    w_type: WeaponType,
    name: String,
    strength: i32,
    speed: i32,
    range: i32,
    position: Position,
    quality: i32,
    quality_maximum: i32,
}

impl GameObject for Weapon {
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn position(&self) -> Position {
        self.position
    }
    fn draw(&self) -> bool {
        true
    }
    fn is_destroyable(&self) -> bool {
        true
    }
    fn can_be_attacked(&self) -> bool {
        true
    }
}
