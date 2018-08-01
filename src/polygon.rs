// Note: this file should probably go into a "primitives" module.

extern crate glium;
extern crate euclid;
extern crate polygon2;

use glium::{Display, VertexBuffer};
use glium::index::PrimitiveType;
use vertex::Vertex;
use canvas::Shape;
use euclid::Point2D;
use polygon2::triangulate;


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




// This gets a little more interesting: a polygon is basically a
// polyline, except that we triangulate it first.  What I don't like:
// there's still a high boiler-plate-to-meaningful-code ratio, but I
// guess it's still not terrible for an ahead-of-time compilation.
//
// I don't like is that I had to switch to nightly rust because the
// polygon2 library gratuitously uses a feature flag for stuff which
// is part of std. If this wasn't a toy graphics library, it would
// really bother me that I had to do that.
//
// I also don't like that we're duplicating vertex data, rather than
// using an index buffer. But glium::index::IndicesSource takes a
// lifetime parameter, and the resulting proliferation of lifetime
// parameters produced so many inscrutable compiler errors that I gave
// up.

pub struct Polygon {
    data: Vec<Vertex>,
}


impl Polygon {
    pub fn new(vertices: &[Vertex]) -> Polygon {
        // Duplicate all the all the vertex data, but as slices!
        let slices: Vec<[f32; 2]> = vertices
            .iter()
            .map(|el| el.position)
            .collect();

        // Find a triangulation for whatever mess of points we got.
        // Duplicate the data back to vertices again, but moreso!
        let triangles: Vec<Vertex> = triangulate(&slices)
            .iter()
            .map(|&i| vertices[i as usize])
            .collect();

        Polygon {
            // Is rust smart enough to elide the copy here? I have no idea.
            data: triangles,
        }
    }
}


impl Shape for Polygon {
    fn get_vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex> {
        VertexBuffer::new(
            display,
            &self.data
        ).unwrap()
    }

    fn get_primitive_type(&self) -> PrimitiveType {
        PrimitiveType::TrianglesList
    }
}
