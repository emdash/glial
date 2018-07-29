#[macro_use] extern crate glium;
extern crate euclid;

pub mod clock;
pub mod vertex;
pub mod layer;
pub mod polyline;
pub mod viewport;


use glium::glutin;
use clock::Clock;
use layer::Layer;

// Quits mainloop when window closes.
// Pretty clear that this was broken before, since the event wasn't
// dispatched to all layers -- each event was round-robined between layers.
fn quit_on_close(event: glutin::Event) -> bool {
    match event {
        glutin::Event::WindowEvent {event, ..} => match event {
            glutin::WindowEvent::CloseRequested => true,
            _ => false,
        },
        _ => false,
    }
}


// this function should probably be generic over the layers and event
// handler collection type.
pub fn render(
    // layers could be any collection of Layers that can traversed in
    // a stable order, but a slice is used for now.
    layers: &[&Layer],
    display: &glium::Display,
    mainloop: &mut glutin::EventsLoop
) {
    let mut closed = false;
    let clock = Clock::new();

    while !closed {
        let time = clock.seconds();
        let scale = time.sin();

        let mut target = display.draw();
        for layer in layers {
            layer.draw(&mut target);
        }
        target.finish().unwrap();

        mainloop.poll_events(|e| {
            // this suggests perhaps an enum return type, indicating
            // the action to take next, redux-style
            //
            // we could also be passing in a set of event handlers to
            // try, in order.
            closed = quit_on_close(e);
            // stores and transformers would be much easier to
            // implement in rust with match expressions.
        });
    }
}

