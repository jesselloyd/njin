use common::game_object::*;

pub struct Scene<'a> {
    pub actors: Vec<Box<Moves + 'a>>,
    pub props: Vec<Box<GameObject + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new(actors: Vec<Box<Moves>>, props: Vec<Box<GameObject>>) -> Scene<'a> {
        Scene {
            actors: actors,
            props: props,
        }
    }

    pub fn add_actor<T: Moves + 'a>(&mut self, actor: Box<T>) {
        self.actors.push(actor)
    }

    pub fn add_prop<T: GameObject + 'a>(&mut self, prop: Box<T>) {
        self.props.push(prop)
    }
}
