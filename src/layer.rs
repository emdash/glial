extern crate glium;

use glium::glutin;
use glium::Surface;


// Abstract a bunch of Drawing-related stuff from the boilerplate.
pub trait Layer {
    fn draw(&self, frame: &mut glium::Frame, time: f32);
    fn handle_event(&self, event: glutin::Event) -> bool;
}


// A Layer which simply clears the background to a solid color.
pub struct ClearColorRGBA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Layer for ClearColorRGBA {
    fn draw(&self, frame: &mut glium::Frame, _: f32) {
        frame.clear_color(self.red, self.green, self.blue, 1.0);
    }

    fn handle_event(&self, _: glutin::Event) -> bool {
        false
    }
}
