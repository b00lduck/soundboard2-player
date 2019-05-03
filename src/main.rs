use glium::{glutin, Surface};
use rodio::Sink;
use std::fs::File;
use std::io::BufReader;
use rodio::source::Source;

fn main() {

    let mut events_loop = glium::glutin::EventsLoop::new();
    let wb = glium::glutin::WindowBuilder::new().with_title("Hello");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let device = rodio::default_output_device().unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.2, 0.2, 1.0, 0.2);
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput{device_id, input} => match input.state {
                        glutin::ElementState::Pressed => match input.virtual_keycode {
                            Some(glutin::VirtualKeyCode::A) => play_sound(&device, "data/test.mp3".to_string()),
                            _ => (),
                        } 
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

fn play_sound(device: &rodio::Device, name: String) {
    let file = File::open(name).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    rodio::play_raw(&device, source.convert_samples());
}