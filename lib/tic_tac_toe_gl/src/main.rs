extern crate tic_tac_toe;
extern crate glutin;
extern crate cpal;

use glutin::WindowBuilder;

fn main() {
    let window = WindowBuilder::new()
        .with_title("Tic-Tac-Toe")
        .with_dimensions(500, 500)
        .build()
        .unwrap();

    let _ = unsafe { window.make_current() };

    for event in window.wait_events() {
        let _ = window.swap_buffers();

        println!("{:?}", event);

        match event {
            glutin::Event::Closed => break,
            _ => (),
        }
    }
}
