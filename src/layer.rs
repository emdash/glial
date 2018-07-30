extern crate glium;
use glium::Surface;
use viewport::ViewPort;


// Abstract a bunch of Drawing-related stuff from the boilerplate.
pub trait Layer {
    fn draw(&self, frame: &mut glium::Frame, vp: &ViewPort);
}


// A Layer which simply clears the background to a solid color.
pub struct ClearColorRGBA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Layer for ClearColorRGBA {
    fn draw(&self, frame: &mut glium::Frame, _: &ViewPort) {
        frame.clear_color(self.red, self.green, self.blue, 1.0);
    }
}
