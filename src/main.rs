#[macro_use] extern crate glium;
extern crate glium_tut;

use glium::glutin;
use glium::Surface;
use glium_tut::render;
use glium_tut::layer::{Layer, ClearColorRGBA};
use glium_tut::vertex::Vertex;


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



// SineWaveDemo generalizes into PolyLine.
// Note, this struct itself is probably more general still.
pub struct PolyLine {
    vbo: glium::VertexBuffer<Vertex>,
    program: glium::Program,
}

// Only this impl, which makes assumptions about the vertex buffer and
// shaders.
impl PolyLine {
    fn new(display: &glium::Display, vertices: &[Vertex]) -> PolyLine {
        PolyLine {
            vbo: glium::VertexBuffer::new(
                display,
                vertices
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


// Even the Layer impl is pretty generic.
impl Layer for PolyLine {
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

    // And it's pretty clear that handle event may not belong in here, now.
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



// Generate some data. Later this will be loaded off disk.

// Note: this internally normalizes into the GL coordinates of (-1,
// -1) to (1, 1). The next step is to factor out the evaluation from
// the transform.
fn evaluate(
    f: &Fn(f32) -> f32,
    domain: [f32; 2],
    n: u32
) -> Vec<Vertex> {
    let x0 = domain[0];

    let span = (domain[1] - domain[0]).abs();

    let x_step = span / (n as f32);
    let scale = 2.0 / span;
    let mut ret = Vec::new();

    for i in 0..n {
        let x = x0 + (i as f32) * x_step;
        ret.push(Vertex {
            position: [x * scale, f(x)],
        });
    }

    ret
}


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let background = ClearColorRGBA {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    let points = evaluate(&|x: f32| x.sin(), [-10.0, 10.0], 100);
    let curve = PolyLine::new(&display, &points);
    let layers: Vec<&Layer> = vec![&background, &curve];

    render(&layers, &display, &mut events_loop);
}
