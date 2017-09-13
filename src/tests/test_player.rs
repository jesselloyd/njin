use super::super::player::Player;
use super::super::common::game_object::*;

#[test]
fn is_facing() {
    let p1 = Player::new("Jesse", 55, 55, Position { x: 1, y: 2 });
    let p2 = Player::new("Asher", 45, 60, Position { x: 1, y: 10 });

    assert!(!p1.is_facing(&p2));
}
