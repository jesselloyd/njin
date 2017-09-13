use terminal_size::*;
use common::scene::*;

pub struct CLIRenderer {
    scene_grid: Vec<Vec<char>>,
}

struct SceneTile {
    x: usize,
    y: usize,
    content: char,
}

impl CLIRenderer {
    pub fn new() -> CLIRenderer {
        let mut renderer = CLIRenderer { scene_grid: vec![vec![]] };
        renderer.clear_buffer();
        renderer
    }

    fn buffer_size(&self) -> (u16, u16) {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            println!("Buffer Size: ({}, {})", w, h);
            (w, h)
        } else {
            (0, 0)
        }
    }

    fn clear_buffer(&mut self) -> (u16, u16) {
        print!("{}[2J", 27 as char);
        let (width, height) = self.buffer_size();
        for h in 0..height {
            self.scene_grid.push(vec![]);
            for _ in 0..width {
                self.scene_grid[h as usize].push(' ');
            }
        }
        (width, height)
    }

    fn load_to_buffer(&mut self, data: SceneTile) {
        self.scene_grid[data.y][data.x] = data.content;
    }

    pub fn draw_scene(&mut self, scene: &Scene) {
        let (width, height) = self.clear_buffer();
        let width = width as i32;
        let height = height as i32;
        for prop in scene.props.iter() {
            if prop.position().x < width && prop.position().y < height {
                self.load_to_buffer(SceneTile {
                    x: prop.position().x as usize,
                    y: prop.position().y as usize,
                    content: 'O',
                });
            }
        }

        for actor in scene.actors.iter() {
            if actor.position().x < width && actor.position().y < height {
                self.load_to_buffer(SceneTile {
                    x: actor.position().x as usize,
                    y: actor.position().y as usize,
                    content: 'I',
                });
            }
        }

        for h in 0..height - 1 {
            for w in 0..width - 1 {
                print!("{}", self.scene_grid[h as usize][w as usize]);
            }
            println!("");
        }
    }
}
