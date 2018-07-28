// Note: this file should probably go into a "primitives" module.

extern crate glium;

use glium::glutin;
use glium::Surface;
use vertex::Vertex;
use layer::Layer;


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


// This struct hints at a more general representation.
// We're really just using nominal typing for dynamic dispatch.
// Perhaps PolyLine can delegate to a more general type.
pub struct PolyLine {
    vbo: glium::VertexBuffer<Vertex>,
    program: glium::Program,
}

impl PolyLine {
    // Having to pass display may be problematic.
    pub fn new(display: &glium::Display, vertices: &[Vertex]) -> PolyLine {
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
