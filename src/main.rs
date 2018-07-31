extern crate euclid;
extern crate glium;
extern crate glium_tut;

use euclid::Point2D;
use glium::glutin;
use glium_tut::render;
use glium_tut::layer::{Layer, ClearColorRGBA};
use glium_tut::vertex::{Vertex, smallest, largest};
use glium_tut::polyline::PolyLine;
use glium_tut::polygon::Rect;
use glium_tut::canvas::{Canvas2D, Shape};
use glium_tut::viewport::{ViewPort, Interval};


// Generate some data. Later this will be loaded off disk.
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


// This ended upgetting moved here, because I wanted to pass the
// viewport into the PolyLine impl. That way we can mutate viewport
// externally. But I didn't want to couple ViewPort to Vertex or vice
// versa. I don't know if that's something I can really avoid at this
// point. Interval may really be a model concern, it could live
// alongside evaluate.
//
// The notion of "fitting" the viewport to some data could be readily
// implemented inside Viewport, provided we have a way to convert
// Vertex to ModelPoint, por at least Vec<Vertex> to
// Vec<ModelPoint>. Perhaps this could be done via a trait.
pub fn fit_viewport_to_data(vertices: &[Vertex]) -> ViewPort {
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

    let sin_x = evaluate(
        &|x: f32| x.sin(),
        [-10.0, 10.0],
        10000
    );

    let cos_x = evaluate(
        &|x: f32| x.cos(),
        [-5.0, 5.0],
        10000
    );

    let vp = fit_viewport_to_data(&sin_x);
    let rect = Rect::new(
        Point2D::new(-2.5, 0.5),
        Point2D::new(2.5, -0.5)
    );

    // Note thi is now copying points. Probably we'll implement
    // zoom_to_fit() on canvas, or viewport now that a clear pattern
    // is emerging WRT where Point2D will be introduced into the API.
    let sin = PolyLine::new(&sin_x);
    let cos = PolyLine::new(&cos_x);
    let shapes: Vec<&Shape> = vec![&sin, &cos, &rect];
    let canvas = Canvas2D::new(&display, &shapes);
    let layers: Vec<&Layer> = vec![&background, &canvas];

    render(&layers, &display, &mut events_loop, &vp);
}
