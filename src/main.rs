#[macro_use]
extern crate glium;
use std::time::Instant;


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


// Wrapper around somewhat obnoxious system time api.
struct Clock {
    instant: Instant,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            instant: Instant::now(),
        }
    }

    // Return system time as floating point value.
    pub fn seconds(&self) -> f32 {
        let e = self.instant.elapsed();
        ((e.as_secs() as f64) + (0.001 * e.subsec_millis() as f64)) as f32
    }
}


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
// I think what this does is arrange for the name "position" to wire
// up to the vertex shader input "position"
implement_vertex!(Vertex, position);


// Generate some data. Later this will be loaded off disk.
fn make_shape(domain: [f32; 2], n: u32) -> Vec<Vertex> {
    let x0 = domain[0];

    let span = (domain[1] - domain[0]).abs();

    let x_step = span / (n as f32);
    let scale = 2.0 / span;
    let mut ret = Vec::new();

    for i in 0..n {
        let x = x0 + (i as f32) * x_step;
        ret.push(Vertex {
            position: [x * scale, x.sin()],
        });
    }

    ret
}

fn main() {
    use glium::glutin;
    use glium::Surface;

    let mut events_loop = glutin::EventsLoop::new();
    let mut closed = false;

    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let clock = Clock::new();

    // Gene
    let shape = make_shape([-10.0, 10.0], 100);

    // so.... these things...
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
    let program = glium::Program::from_source(
        &display,
        VERTEX_SHADER_SRC,
        FRAGMENT_SHADER_SRC, None
    ).unwrap();


    while !closed {
        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // ... plus this call ... 
        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniform!{time: clock.seconds()},
            &Default::default()
        ).unwrap();
        // corresponds to a single "layer" in the image.

        // The draw call needs:
        // - vertex buffer
        // - an enum explaining *what* the vertex list represents
        // - the shader programs to interpret the vertex buffer
        // - uniforms (i.e., input and output)

        // The extension points are:
        // - the shape of the Vertex object.
        //   - i.e. arbitrary named per-vertex fields, corresponding to glsl "in" parameters.
        //   - i.e. arbitrary named per-vertex fields, corresponding to glsl "attribute" parameters.
        // - the shape of the uniforms object
        //   - i.e. arbitrary named per-uniform fields, corresponding to glsl "uniform" parameters.

        // Glium is able to use macros and or RTTI to automatically
        // connect these things together. The underlying opengl C api
        // is string-based and dynamic.

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
