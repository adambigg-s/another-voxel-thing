#![allow(dead_code, unused_variables, unused_mut)]

mod buffer;
mod camera;
mod raymarch;
mod tree;

const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 300;

fn main() {
    let mut window = minifb::Window::new(
        "game",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        minifb::WindowOptions { scale: minifb::Scale::X2, ..Default::default() },
    )
    .unwrap();

    println!("is this working?");
}
