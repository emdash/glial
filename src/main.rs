#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
// I'm guessing this macro implements a trait that Glium understands
// for an arbitrary user-defined type, using the position attribute...
// not sure why they take this approach rather than supplying a vertex type.
implement_vertex!(Vertex, position);

// I'm declaring these as globals, regardless what the tutorial says for now.
const vertex1: Vertex = Vertex {position: [-0.5, -0.5]};
const vertex2: Vertex = Vertex {position: [ 0.5,  0.5]};
const vertex3: Vertex = Vertex {position: [ 0.5, -0.25]};

const vertex_shader_src: &str = r#"
  #version 140
  in vec2 position;
  void main() {
    gl_Position = vec4(position, 0.0, 1.0);
  }
"#;

const fragment_shader_src: &str = r#"
  #version 140
  out vec4 color;
  void main() {
    color = vec4(1.0, 0.0, 0.0, 1.0);
  }
"#;


fn main() {
    use glium::glutin;
    use glium::Surface;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let shape = vec!{vertex1, vertex2, vertex3};
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(
        &display,
        vertex_shader_src,
        fragment_shader_src, None
    ).unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();

        target.finish().unwrap();
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent {event, ..} => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}