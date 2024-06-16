use minifb::{Window, WindowOptions};

use crate::{entity::Entity, util::{pnt_to_buf, FigResult}};

pub struct Canvas {
    buf: Vec<u32>,
    win: Window,
    width: usize,
    height: usize,
    entities: Vec<Entity>,
}

impl Canvas {
    pub fn new(name: &str, dim: (usize, usize), fps: usize) -> FigResult<Self> {
        let mut me = Canvas {
            buf: vec![0; dim.0 * dim.1],
            win: Window::new(name, dim.0, dim.1, WindowOptions::default())?,
            width: dim.0,
            height: dim.1,
            entities: vec![],
        };

        me.win.set_target_fps(fps);

        Ok(me)
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn update(&mut self) -> FigResult<()> {
        for e in self.entities.iter_mut() {
            if e.anim_rate.0 == e.anim_rate.1 {
                e.next_frame();
                e.anim_rate.1 = 0;
            } else {
                e.anim_rate.1 += 1;
            }

            let frame = e.get_frame();
            for (i, pix) in frame.iter().enumerate() {
                let origin = pnt_to_buf(e.pos, self.width);
                let buf_pos = origin + (self.width * (i / e.dim.0)) + (i % e.dim.0);
                self.buf[buf_pos] = *pix;
            }
        }

        self.win.update_with_buffer(&self.buf, self.width, self.height)?;

        Ok(())
    }

    pub fn is_open(&self) -> bool {
        return self.win.is_open();
    }
}
