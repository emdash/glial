extern crate glium;
extern crate glium_tut;

use glium::glutin;
use glium_tut::render;
use glium_tut::layer::{Layer, ClearColorRGBA};
use glium_tut::vertex::Vertex;
use glium_tut::polyline::PolyLine;


// Generate some data. Later this will be loaded off disk.

// Note: this internally normalizes into the GL coordinates of (-1,
// -1) to (1, 1). The next step is to factor out the evaluation from
// the transform.
fn evaluate(
    f: &Fn(f32) -> f32,
    domain: [f32; 2],
    n: u32
) -> Vec<Vertex> {
    let x0 = domain[0];
    let span = (domain[1] - domain[0]).abs();
    let x_step = span / (n as f32);
    let mut ret = Vec::new();

    for i in 0..n {
        let x = x0 + (i as f32) * x_step;
        ret.push(Vertex {
            position: [x, f(x)],
        });
    }

    ret
}


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let background = ClearColorRGBA {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    let points = evaluate(&|x: f32| 10.0 * (x * 5.0).sin() - 5.0, [-13.5, 50.0], 1000);
    let curve = PolyLine::new(&display, &points);
    let layers: Vec<&Layer> = vec![&background, &curve];

    render(&layers, &display, &mut events_loop);
}
