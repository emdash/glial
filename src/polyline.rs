// Note: this file should probably go into a "primitives" module.

extern crate glium;

use glium::{Surface};
use vertex::Vertex;
use layer::Layer;
use viewport::{ViewPort, Interval};
use std;


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
    viewport: ViewPort,
}

// And now the problem of floats not implmenting Ord rears its ugly
// head.  Will solve this with an external crate later. For now just
// do something naive to get us going.
fn smallest(vertices: &[Vertex], index: usize) -> f32 {
    vertices.iter().map(|el| el.position[index]).fold(
        std::f32::MAX,
        |a, b| a.min(b)
    )
}


fn largest(vertices: &[Vertex], index: usize) -> f32 {
    vertices.iter().map(|el| el.position[index]).fold(
        std::f32::MIN,
        |a, b| a.max(b)
    )
}

pub fn fit_to_data(vertices: &[Vertex]) -> ViewPort {
    ViewPort::new(
        Interval::from_endpoints(
            smallest(vertices, 0),
            largest(vertices, 0)

        ),
        Interval::from_endpoints(
            smallest(vertices, 1),
            largest(vertices, 1),
        )
    )
}

fn to_gl_array(vp: &ViewPort) -> [[f32; 3]; 3] {
    let slice = vp.get_transform().to_row_arrays();

    [
        [slice[0][0], slice[1][0], 0.0],
        [slice[1][0], slice[1][1], 0.0],
        [slice[2][0], slice[2][1], 1.0],
    ]
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
            viewport: fit_to_data(vertices)
        }
    }
}


// Even the Layer impl is pretty generic.
impl Layer for PolyLine {
    fn draw(&self, frame: &mut glium::Frame) {
        let indexes = glium::index::NoIndices(
            glium::index::PrimitiveType::LineStrip
        );

        let uniforms = uniform! {
            transform: to_gl_array(&self.viewport)
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
