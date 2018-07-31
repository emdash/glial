// Note: this file should probably go into a "primitives" module.

extern crate glium;

use glium::{Surface, VertexBuffer, Display, Program};
use glium::index::{PrimitiveType, NoIndices};
use vertex::Vertex;
use layer::Layer;
use viewport::ViewPort;


// A shape is something which can be rendered into a Canvas
pub trait Shape {
    fn get_vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex>;
    fn get_primitive_type(&self) -> PrimitiveType;
}


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


// This shader is just a solid color.
// TODO: per-object color attribute.
const FRAGMENT_SHADER_SRC: &str = r#"
  #version 140
  out vec4 color;
  void main() {
    color = vec4(1.0, 1.0, 0.0, 1.0);
  }
"#;


// A Canvas2D contains an arbitrary number of geometric primitives.
pub struct Canvas2D {
    geometry: Vec<(VertexBuffer<Vertex>, PrimitiveType)>,
    program: Program,
}


// We precompute the geometry for all the shapes in the canvas.
impl Canvas2D {
    // Having to pass display may be problematic.
    pub fn new(display: &Display, shapes: &[&Shape]) -> Canvas2D {
        // We might or might not want to defer this step to the draw
        // method. For now let's assume geometry is static. Later we
        // can push this into a "rebuild" operation.
        let geometry = shapes.iter().map(|shape| {
            (
                shape.get_vertex_buffer(display),
                shape.get_primitive_type()
            )
        }).collect();

        Canvas2D {
            geometry: geometry,
            program: Program::from_source(
                display,
                VERTEX_SHADER_SRC,
                FRAGMENT_SHADER_SRC,
                None
            ).unwrap(),
        }
    }
}


// A Canvas is a Layer, and knows how to draw itself into the current context.
impl Layer for Canvas2D {
    fn draw(&self, frame: &mut glium::Frame, vp: &ViewPort) {
        let uniforms = uniform! {
            transform: vp.to_gl_array()
        };

        for (vbo, primitive_type) in &self.geometry {
            frame.draw(
                vbo,
                &NoIndices(*primitive_type),
                &self.program,
                &uniforms,
                &Default::default()
            ).unwrap();
        }
    }
}
