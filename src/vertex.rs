use std;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);


// And now the problem of floats not implmenting Ord rears its ugly
// head.  Will solve this with an external crate later. For now just
// do something naive to get us going.
pub fn smallest(vertices: &[Vertex], index: usize) -> f32 {
    vertices.iter().map(|el| el.position[index]).fold(
        std::f32::MAX,
        |a, b| a.min(b)
    )
}


pub fn largest(vertices: &[Vertex], index: usize) -> f32 {
    vertices.iter().map(|el| el.position[index]).fold(
        std::f32::MIN,
        |a, b| a.max(b)
    )
}
