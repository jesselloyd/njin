use super::super::common::game_object::*;
use super::super::common::scene::*;
use super::super::player::*;
use super::super::renderer::cli::CLIRenderer;

#[test]
fn can_render_50x50() {
    let p1 = Player::new("Jesse", 55, 55, Position { x: 4, y: 2 });
    let p2 = Player::new("Asher", 45, 60, Position { x: 25, y: 10 });
    let p3_fake = Player::new("Fake_OBJ", 45, 60, Position { x: 5, y: 20 });


    let mut scene = Scene::new(vec![], vec![]);
    scene.actors.push(Box::new(p1));
    scene.actors.push(Box::new(p2));
    scene.props.push(Box::new(p3_fake));

    let mut renderer = CLIRenderer::new();
    renderer.draw_scene(&scene);
}
