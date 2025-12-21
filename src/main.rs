use minifb::Key;

mod buffer;
mod camera;
mod wall;

const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 300;
const WINDOW_CLEAR_COLOR: u32 = (0xffu32 << 24) | (15u32 << 16) | (25u32 << 8) | 40u32;

fn main() {
    let mut window = minifb::Window::new(
        "doom game",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        minifb::WindowOptions { scale: minifb::Scale::X4, ..Default::default() },
    )
    .unwrap();
    window.set_target_fps(120usize);
    let mut camera = camera::Camera::build(90f32);
    camera.look_speed = 0.0005f32;
    camera.move_speed = 0.0005f32;
    camera.fov = 90.0f32;
    let mut buffer = buffer::Buffer::build(0u32, [WINDOW_WIDTH, WINDOW_HEIGHT]);

    let mut walls = Vec::new();
    walls.extend_from_slice(&[
        wall::Wall {
            p1: glam::ivec2(-10, 10),
            p2: glam::ivec2(10, 10),
            h: 20,
            col: 0xff00ffff,
        },
        wall::Wall {
            p1: glam::ivec2(10, 10),
            p2: glam::ivec2(10, -30),
            h: 40,
            col: 0xffffaa00,
        },
    ]);

    loop {
        if window.is_key_down(Key::Escape) {
            break;
        }
        buffer.fill(WINDOW_CLEAR_COLOR);

        for wall in &walls {
            wall::render_wall(&mut buffer, &camera, wall);
        }

        camera::minifb_input_cb(&window, &mut camera);
        camera.display();
        window.update_with_buffer(&buffer.items, buffer.size[0], buffer.size[1]).unwrap();
    }
}
