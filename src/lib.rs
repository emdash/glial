pub mod clock;
pub mod layer;

extern crate glium;

use glium::glutin;
use glium::Surface;
use clock::Clock;
use layer::Layer;

pub fn render(
    layers: &[&Layer],
    display: &glium::Display,
    mainloop: &mut glutin::EventsLoop
) {
    let mut closed = false;
    let clock = Clock::new();

    while !closed {
        let mut target = display.draw();

        for layer in layers {
            layer.draw(&mut target, clock.seconds());

            mainloop.poll_events(|e| {
                closed = layer.handle_event(e);
            });
        }

        target.finish().unwrap();
    }
}
