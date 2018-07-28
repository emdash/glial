#[macro_use]
extern crate glium;
use std::time::Instant;
use glium::glutin;
use glium::Surface;



// Shaders -- rapidly zoom in and out.
const VERTEX_SHADER_SRC: &str = r#"
  #version 140
  in vec2 position;
  uniform float time;
  void main() {
    gl_Position = vec4(position * sin(time), 0.0, 1.0);
  }
"#;

// Shaders -- oscillate between yellow and green.
const FRAGMENT_SHADER_SRC: &str = r#"
  #version 140
  out vec4 color;
  uniform float time;
  void main() {
    color = vec4((sin(time) + 1) * 0.5, 1.0, 0.0, 1.0);
  }
"#;


// Wrapper around somewhat obnoxious system time api.
struct Clock {
    instant: Instant,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            instant: Instant::now(),
        }
    }

    // Return system time as floating point value.
    pub fn seconds(&self) -> f32 {
        let e = self.instant.elapsed();
        ((e.as_secs() as f64) + (0.001 * e.subsec_millis() as f64)) as f32
    }
}


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
// I think what this does is arrange for the name "position" to wire
// up to the vertex shader input "position"
implement_vertex!(Vertex, position);


// Abstract a bunch of Drawing-related stuff from the boilerplate.
pub trait Layer {
    fn draw(&self, frame: &mut glium::Frame, time: f32);
    fn handle_event(&self, event: glutin::Event) -> bool;
}


// Factor out the original demo code into a struct that implements the Layer trait
pub struct SineWaveDemo {
    vbo: glium::VertexBuffer<Vertex>,
    program: glium::Program,
}

impl SineWaveDemo {
    fn new(display: &glium::Display) -> SineWaveDemo {
        let vertices = make_shape([-10.0, 10.0], 100);

        SineWaveDemo {
            vbo: glium::VertexBuffer::new(
                display,
                &vertices
            ).unwrap(),
            program: glium::Program::from_source(
                display,
                VERTEX_SHADER_SRC,
                FRAGMENT_SHADER_SRC,
                None
            ).unwrap()
        }
    }
}

impl Layer for SineWaveDemo {
    fn draw(&self, frame: &mut glium::Frame, time: f32) {
        let indexes = glium::index::NoIndices(
            glium::index::PrimitiveType::LineStrip
        );

        frame.draw(
            &self.vbo,
            &indexes,
            &self.program,
            &uniform!{time: time},
            &Default::default()
        ).unwrap();
    }

    fn handle_event(&self, event: glutin::Event) -> bool {
        match event {
            glutin::Event::WindowEvent {event, ..} => match event {
                glutin::WindowEvent::CloseRequested => true,
                _ => false,
            },
            _ => false,
        }
    }
}


pub struct ClearColorRGBA {
    red: f32,
    green: f32,
    blue: f32,
    // XXX: is alpha meaningful for clear_color?
    alpha: f32,
}


impl Layer for ClearColorRGBA {
    fn draw(&self, frame: &mut glium::Frame, _: f32) {
        frame.clear_color(self.red, self.green, self.blue, self.alpha);
    }

    fn handle_event(&self, _: glutin::Event) -> bool {
        false
    }
}


// Generate some data. Later this will be loaded off disk.
fn make_shape(domain: [f32; 2], n: u32) -> Vec<Vertex> {
    let x0 = domain[0];

    let span = (domain[1] - domain[0]).abs();

    let x_step = span / (n as f32);
    let scale = 2.0 / span;
    let mut ret = Vec::new();

    for i in 0..n {
        let x = x0 + (i as f32) * x_step;
        ret.push(Vertex {
            position: [x * scale, x.sin()],
        });
    }

    ret
}


fn render(
    layers: &[&Layer],
    display: &glium::Display,
    mainloop: &mut glutin::EventsLoop
) {
    let mut closed = false;
    let clock = Clock::new();

    while !closed {
        let mut target = display.draw();

        for layer in layers {
            layer.draw(&mut target, clock.seconds());

            mainloop.poll_events(|e| {
                closed = layer.handle_event(e);
            });
        }

        target.finish().unwrap();
    }
}


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let background = ClearColorRGBA {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0
    };

    let sine = SineWaveDemo::new(&display);

    let layers: Vec<&Layer> = vec![&background, &sine];

    render(&layers, &display, &mut events_loop);
}
