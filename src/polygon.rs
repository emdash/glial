// Note: this file should probably go into a "primitives" module.

extern crate glium;
extern crate euclid;

use glium::{Display, VertexBuffer};
use glium::index::PrimitiveType;
use vertex::Vertex;
use canvas::Shape;
use euclid::Point2D;


pub struct Rect {
    // hm, this looks suspiciously familiar...
    data: Vec<Vertex>
}


impl Rect {
    pub fn new(top_left: Point2D<f32>, bottom_right: Point2D<f32>) -> Rect {
        let top_right = Point2D::new(bottom_right.x, top_left.y);
        let bottom_left = Point2D::new(top_left.x, bottom_right.y);
        Rect {
            data: vec![
                Vertex {position: [bottom_left.x, bottom_left.y]},
                Vertex {position: [top_left.x, top_left.y]},
                Vertex {position: [bottom_right.x, bottom_right.y]},
                Vertex {position: [top_right.x, top_right.y]},
            ]
        }
    }
}


// Even the Layer impl is pretty generic.
impl Shape for Rect {
    // ...As does this....
    fn get_vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex> {
         glium::VertexBuffer::new(
             display,
             &self.data
         ).unwrap()
    }

    fn get_primitive_type(&self) -> PrimitiveType {
        // is this the only thing that changes??
        PrimitiveType::TriangleStrip
    }
}
