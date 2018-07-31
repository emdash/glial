// Note: this file should probably go into a "primitives" module.

extern crate glium;

use glium::{Display, VertexBuffer};
use glium::index::PrimitiveType;
use vertex::Vertex;
use canvas::Shape;


pub struct PolyLine {
    data: Vec<Vertex>,
}


impl PolyLine {
    // I think at this point it makes sense for primitives to take
    // ModelPoint directly, and perhaps it makes sense for them also
    // to depend on ViewPort. We'll save that for the next refactoring.
    pub fn new(vertices: &[Vertex]) -> PolyLine {
        PolyLine {
            data: vertices.to_vec(),
        }
    }
}


// Even the Layer impl is pretty generic.
impl Shape for PolyLine {
    fn get_vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex> {
         glium::VertexBuffer::new(
             display,
             &self.data
         ).unwrap()
    }

    fn get_primitive_type(&self) -> PrimitiveType {
        PrimitiveType::LineStrip
    }
}
