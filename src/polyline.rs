// Note: this file should probably go into a "primitives" module.

extern crate glium;

use glium::{Surface};
use vertex::Vertex;
use layer::Layer;
use viewport::ViewPort;


// This shader basically just represents the 2D viewport.
// A lot of geometric primitives would re-use this.
const VERTEX_SHADER_SRC: &str = r#"
  #version 140
  in vec2 position;
  uniform mat3 transform;
  void main() {
    gl_Position = vec4(transform * vec3(position, 1.0), 1.0);
  }
"#;


// This shader is just a solid color. We just lost the uniform we were
// using to vary the color over time.
//
// The fragment shader will correspond to a material and / or pattern.
const FRAGMENT_SHADER_SRC: &str = r#"
  #version 140
  out vec4 color;
  void main() {
    color = vec4(1.0, 1.0, 0.0, 1.0);
  }
"#;


// This struct hints at a more general representation.
// We're really just using nominal typing for dynamic dispatch.
// Perhaps PolyLine can delegate to a more general type.
pub struct PolyLine {
    vbo: glium::VertexBuffer<Vertex>,
    // This program really belongs to a more generic notion of a "2D
    // Canvas", or "2D Primative", of which PolyLine is just one
    // instance. All primatives would render into a viewport, with
    // their per-instance pattern.
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
            ).unwrap(),
        }
    }
}


// Even the Layer impl is pretty generic.
impl Layer for PolyLine {
    fn draw(&self, frame: &mut glium::Frame, vp: &ViewPort) {
        let indexes = glium::index::NoIndices(
            glium::index::PrimitiveType::LineStrip
        );

        let uniforms = uniform! {
            transform: vp.to_gl_array()
        };

        frame.draw(
            &self.vbo,
            &indexes,
            &self.program,
            &uniforms,
            &Default::default()
        ).unwrap();
    }
}
