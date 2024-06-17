use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use minifb::{Window, WindowOptions};

use crate::entity::Entity;
use crate::input::InputManager;
use crate::util::{pnt_to_buf, FigResult};

pub struct Canvas<'a> {
    buf: Vec<u32>,
    win: Window,
    width: usize,
    height: usize,
    entities: HashMap<String, Rc<RefCell<Entity>>>,
    pub input: InputManager<'a>,
}

impl<'a> Canvas<'a> {
    pub fn new(name: &str, dim: (usize, usize), fps: usize) -> FigResult<Self> {
        let mut me = Canvas {
            buf: vec![0; dim.0 * dim.1],
            win: Window::new(name, dim.0, dim.1, WindowOptions::default())?,
            width: dim.0,
            height: dim.1,
            entities: HashMap::new(),
            input: InputManager::new(),
        };

        me.win.set_target_fps(fps);

        Ok(me)
    }

    pub fn add_entity(&mut self, name: &str, entity: Rc<RefCell<Entity>>) {
        self.entities.insert(String::from(name), entity);
    }

    
    pub fn mov_entity(&mut self, name: &str, dx: f32, dy: f32) {
        self.entities.get_mut(name).expect("oops").borrow_mut().mov(dx, dy);
    }

    pub fn update(&mut self) -> FigResult<()> {
        self.buf = vec![0; self.width * self.height];

        for (_, e) in self.entities.iter_mut() {
            if e.borrow().anim_rate.0 == e.borrow().anim_rate.1 {
                e.borrow_mut().next_frame();
                e.borrow_mut().anim_rate.1 = 0;
            } else {
                e.borrow_mut().anim_rate.1 += 1;
            }

            let frame = e.borrow().get_frame();
            for (i, pix) in frame.iter().enumerate() {
                let origin = pnt_to_buf(e.borrow().pos, self.width);
                let buf_pos = origin + (self.width * (i / e.borrow().dim.0)) + (i % e.borrow().dim.0);
                self.buf[buf_pos] = *pix;
            }
        }

        for (key, val) in self.input.keybinds.iter_mut() {
            if self.win.is_key_down(*key) {
                val();
            }
        }

        self.win.update_with_buffer(&self.buf, self.width, self.height)?;

        Ok(())
    }

    pub fn is_open(&self) -> bool {
        return self.win.is_open();
    }
}
