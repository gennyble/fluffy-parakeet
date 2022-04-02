use std::rc::Rc;

use glow::HasContext;
use glutin::{window::Window, ContextWrapper, PossiblyCurrent};

use crate::color::Rgba;

pub struct OpenGl {
    gl: Rc<glow::Context>,
    clear_color: Rgba,
}

impl OpenGl {
    pub fn new(context: &ContextWrapper<PossiblyCurrent, Window>) -> Self {
        let gl = unsafe {
            glow::Context::from_loader_function(|s| context.get_proc_address(s) as *const _)
        };

        let mut this = Self {
            gl: Rc::new(gl),
            clear_color: Rgba::new(0.0, 0.0, 0.0, 1.0),
        };

        // Set up some things
        this.clear_color(this.clear_color);

        this
    }

    pub fn clear_color<C: Into<Rgba>>(&mut self, color: C) {
        let c = color.into();
        self.clear_color = c;
        unsafe { self.gl.clear_color(c.r, c.g, c.b, c.a) }
    }

    pub fn clear(&self) {
        unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT) }
    }
}
