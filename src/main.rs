mod buffer;
mod camera;

use minifb::Key;

use crate::{buffer::Buffer, camera::Camera};

const WIDTH: usize = 150;
const HEIGHT: usize = 100;
const VSIZE: usize = 64;

fn main() {
    let mut window = minifb::Window::new(
        "voxel",
        WIDTH,
        HEIGHT,
        minifb::WindowOptions { scale: minifb::Scale::X4, ..Default::default() },
    )
    .unwrap();
    let mut buffer = Buffer::build(0u32, [WIDTH, HEIGHT]);
    let mut camera = Camera::build(WIDTH as f32 / HEIGHT as f32, 90.0);
    camera.look_speed = 0.01;
    camera.move_speed = 0.01;
    let mut vox = VoxelList::new(glam::USizeVec3::new(VSIZE, VSIZE, VSIZE));

    loop {
        if window.is_key_down(Key::Escape) {
            break;
        }

        camera::minifb_input_cb(&window, &mut camera);
        println!("{}", &camera);

        buffer.fill(0);
        render_scene(&camera, &vox, &mut buffer);
        window.update_with_buffer(&buffer.items, buffer.size[0], buffer.size[1]).unwrap();
    }
}

fn render_scene(camera: &Camera, voxels: &VoxelList, buff: &mut Buffer<u32, 2>) {
    fn pack_color(color: glam::U8Vec3) -> u32 {
        let [r, g, b] = color.to_array();
        0xff | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }
}

struct RayHit {
    color: glam::U8Vec3,
}

struct Ray {
    orig: glam::Vec3A,
    dir: glam::Vec3A,
}

trait RayMarch {
    fn march(&self, ray: Ray, tmax: f32) -> Option<RayHit>;
}

struct VoxelList {
    presence: Buffer<bool, 3>,
    colors: Buffer<glam::U8Vec3, 3>,
    size: glam::USizeVec3,
}

impl RayMarch for VoxelList {
    fn march(&self, ray: Ray, tmax: f32) -> Option<RayHit> {
        todo!()
    }
}

impl VoxelList {
    fn new(size: glam::USizeVec3) -> Self {
        Self {
            presence: Buffer::build(false, size.to_array()),
            colors: Buffer::build(glam::U8Vec3::default(), size.to_array()),
            size,
        }
    }

    fn sample(&self, location: glam::USizeVec3) -> Option<RayHit> {
        todo!()
    }

    fn set(&mut self, location: glam::USizeVec3, color: glam::U8Vec3) {
        self.colors[location.to_array()] = color;
    }
}
