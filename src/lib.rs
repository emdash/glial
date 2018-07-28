pub mod clock;

extern crate glium;

use glium::glutin;
use glium::Surface;
use clock::Clock;

// Abstract a bunch of Drawing-related stuff from the boilerplate.
pub trait Layer {
    fn draw(&self, frame: &mut glium::Frame, time: f32);
    fn handle_event(&self, event: glutin::Event) -> bool;
}

pub struct ClearColorRGBA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    // XXX: is alpha meaningful for clear_color?
    pub alpha: f32,
}

impl Layer for ClearColorRGBA {
    fn draw(&self, frame: &mut glium::Frame, _: f32) {
        frame.clear_color(self.red, self.green, self.blue, self.alpha);
    }

    fn handle_event(&self, _: glutin::Event) -> bool {
        false
    }
}

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


