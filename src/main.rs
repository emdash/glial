#[macro_use]
extern crate glium;

use std::time::Instant;

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
// I'm guessing this macro implements a trait that Glium understands
// for an arbitrary user-defined type, using the position attribute...
// not sure why they take this approach rather than supplying a vertex type.
implement_vertex!(Vertex, position);


const VERTEX_SHADER_SRC: &str = r#"
  #version 140
  in vec2 position;
  uniform float time;
  void main() {
    gl_Position = vec4(position * sin(time), 0.0, 1.0);
  }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
  #version 140
  out vec4 color;
  void main() {
    color = vec4(1.0, 0.0, 0.0, 1.0);
  }
"#;


// Generate some data. Later this will be loaded off disk.
fn make_shape(domain: [f32; 2], n: u32) -> Vec<Vertex> {
    let x0 = domain[0];

    let span = (domain[1] - domain[0]).abs();
    println!("{}", span);

    let x_step = span / (n as f32);
    let scale = 2.0 / span;
    let mut ret = Vec::new();

    for i in 0..n {
        let x = (x0 + (i as f32) * x_step);
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

    let shape = make_shape([-50.0, 50.0], 1000);
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
        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniform!{time: clock.seconds()},
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
