use std::cell::RefCell;
use std::rc::Rc;

use crate::util::Pnt;

pub struct Entity {
    pub pos: Pnt,
    pub frames: Vec<Vec<u32>>,
    pub dim: (usize, usize),
    pub anim_rate: (usize, usize),
    frame: usize,
}

impl Entity {
    pub fn new(pos: Pnt, frames: Vec<Vec<u32>>, dim: (usize, usize), anim_rate: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Entity {
            pos,
            frames: frames.iter().map(|x| x[..dim.0 * dim.1].to_vec()).collect(),
            dim,
            anim_rate: (anim_rate, 0),
            frame: 0,
        }))
    }

    pub fn next_frame(&mut self) {
        self.frame += 1;
        if self.frame == self.frames.len() {
            self.frame = 0;
        }
    }

    pub fn get_frame(&self) -> Vec<u32> {
        return self.frames[self.frame][..].to_vec();
    }

    pub fn mov(&mut self, dx: f32, dy: f32) {
        self.pos.0 += dx;
        self.pos.1 += dy;
    }
}