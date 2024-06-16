use crate::util::Pnt;

pub struct Entity {
    pub pos: Pnt,
    pub frames: Vec<Vec<u32>>,
    pub dim: (usize, usize),
    pub anim_rate: (usize, usize),
    frame: usize,
}

impl Entity {
    pub fn new(pos: Pnt, frames: Vec<Vec<u32>>, dim: (usize, usize), anim_rate: usize) -> Self {
        Entity {
            pos,
            frames,
            dim,
            anim_rate: (anim_rate, 0),
            frame: 0,
        }
    }

    pub fn next_frame(&mut self) {
        self.frame += 1;
        if self.frame == self.frames.len() {
            self.frame = 0;
        }
    }

    pub fn get_frame(&self) -> &Vec<u32> {
        return &self.frames[self.frame];
    }
}