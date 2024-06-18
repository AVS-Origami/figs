use minifb::{Key, Window, WindowOptions};

use crate::entity::Entity;
use crate::util::{pnt_to_buf, FigResult};

pub struct Canvas {
    buf: Vec<u32>,
    win: Window,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(name: &str, dim: (usize, usize), fps: usize) -> FigResult<Self> {
        let mut me = Canvas {
            buf: vec![0; dim.0 * dim.1],
            win: Window::new(name, dim.0, dim.1, WindowOptions::default())?,
            width: dim.0,
            height: dim.1,
        };

        me.win.set_target_fps(fps);

        Ok(me)
    }

    pub fn clear(&mut self) {
        self.buf = vec![0; self.width * self.height];
    }

    pub fn draw(&mut self, e: &Entity) {
        let frame = e.get_frame();
        for (i, pix) in frame.iter().enumerate() {
            let origin = pnt_to_buf(e.pos, self.width);
            let buf_pos = origin + (self.width * (i / e.dim.0)) + (i % e.dim.0);
            self.buf[buf_pos] = *pix;
        }
    }

    pub fn update(&mut self) -> FigResult<()> {
        self.win.update_with_buffer(&self.buf, self.width, self.height)?;
        Ok(())
    }

    pub fn is_open(&self) -> bool {
        return self.win.is_open();
    }

    pub fn key_down(&self, key: Key) -> bool {
        return self.win.is_key_down(key);
    }
}
