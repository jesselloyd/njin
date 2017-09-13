#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Forward,
    Backward,
}

pub trait GameObject {
    fn name(&self) -> String;
    fn position(&self) -> Position;
    fn is_destroyable(&self) -> bool {
        false
    }
    fn can_be_attacked(&self) -> bool {
        false
    }
}

pub trait Moves: GameObject {
    fn movement(&mut self, direction: Direction);
}

pub trait Interacts: Moves {
    fn attack<T: GameObject>(&self, target: &mut T);
    fn interact_with<T: GameObject>(&self, target: &mut T);
    fn toggle_block(&mut self);
    fn is_facing<T: GameObject>(&self, target: &T) -> bool;
    fn in_range_of<T: GameObject>(&self, target: &T) -> bool;
    fn say(&self, phase: &str);
    fn focus_on<T: GameObject>(&self, target: &mut T);
    fn on_death(&mut self);
    fn pick_up<T: GameObject>(&mut self, object: T);
    fn discard<T: GameObject>(&mut self, object: T) -> T;
}
