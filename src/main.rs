use minifb::Key;

mod buffer;
mod camera;

const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 300;

fn main() {
    let mut window = minifb::Window::new(
        "doom game",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        minifb::WindowOptions { scale: minifb::Scale::X4, ..Default::default() },
    )
    .unwrap();
    let mut camera = camera::Camera::build(WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32, 90f32);
    camera.look_speed = 0.0005f32;
    camera.move_speed = 0.0005f32;
    camera.fov = 90.0f32;
    let mut buffer = buffer::Buffer::build(0u32, [WINDOW_WIDTH, WINDOW_HEIGHT]);

    let wall = Wall {
        p1: glam::vec2(-10.0, 5.0),
        p2: glam::vec2(10.0, 5.0),
        h: 10.0,
    };

    loop {
        if window.is_key_down(Key::Escape) {
            break;
        }
        buffer.fill(0u32);

        camera::minifb_input_cb(&window, &mut camera);
        camera.display();
        window.update_with_buffer(&buffer.items, buffer.size[0], buffer.size[1]).unwrap();
    }
}

struct Wall {
    p1: glam::Vec2,
    p2: glam::Vec2,
    h: f32,
}
