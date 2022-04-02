mod color;
mod opengl;

use glutin::{
    dpi::PhysicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowBuilderExtUnix,
    window::{Window, WindowBuilder},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};
use opengl::OpenGl;

/// The main game struct. I know, god objects bad, but like, something has to
/// contain the world. Collapse is the current title of the game.
struct Collapse {
    context: ContextWrapper<PossiblyCurrent, Window>,
    gl: OpenGl,
}

impl Collapse {
    pub fn run() {
        let window_size = PhysicalSize::new(800, 600);

        let el = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title("Collapse")
            .with_app_id("pleasefloat".into())
            .with_inner_size(window_size);
        let wc = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &el)
            .unwrap();
        let context = unsafe { wc.make_current().unwrap() };
        let gl = OpenGl::new(&context);

        let mut this = Self { context, gl };

        println!("Starting to run!");
        el.run(move |event, _, flow| this.event_handler(event, flow))
    }

    pub fn event_handler(&mut self, event: Event<()>, flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { window_id, event } => (),
            Event::MainEventsCleared => {
                self.update();
                self.draw();
                self.context.swap_buffers().unwrap();
            }
            Event::RedrawRequested(_) => {
                self.context.swap_buffers().unwrap();
            }
            Event::LoopDestroyed => todo!(),
            _ => (),
        }
    }

    fn update(&mut self) {}

    fn draw(&mut self) {
        self.gl.clear();
    }
}

fn main() {
    Collapse::run();
}
