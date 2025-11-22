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
    camera.fov = 90.0;
    let mut vox = VoxelList::new(glam::USizeVec3::new(VSIZE, VSIZE, VSIZE));
    vox.set(glam::USizeVec3::new(0, 0, 5), glam::U8Vec3::new(255, 0, 255));
    vox.set(glam::USizeVec3::new(1, 0, 5), glam::U8Vec3::new(255, 255, 0));
    vox.set(glam::USizeVec3::new(0, 1, 5), glam::U8Vec3::new(0, 0, 255));
    let test_point = glam::Vec3A::new(0.0, 0.0, -3.0);

    loop {
        if window.is_key_down(Key::Escape) {
            break;
        }

        buffer.fill(0);
        camera::minifb_input_cb(&window, &mut camera);
        render_scene(&camera, &vox, &mut buffer);
        render_test_point(&camera, test_point, &mut buffer);
        window.update_with_buffer(&buffer.items, buffer.size[0], buffer.size[1]).unwrap();
        println!("{}", camera);
    }
}

fn render_test_point(camera: &Camera, point: glam::Vec3A, buff: &mut Buffer<u32, 2>) {
    let rel = point - camera.pos;

    let vx = camera.rvec.dot(rel);
    let vy = camera.uvec.dot(rel);
    let vz = camera.fvec.dot(rel);

    if vz < 0.0 {
        return;
    }
    let focal = 1.0 / (camera.fov.to_radians() / 2.0).tan();

    let sx = vx / -vz * focal * camera.asp_ratio;
    let sy = vy / -vz * focal;

    let [hw, hh] = buff.size.map(|val| (val / 2) as f32);

    let sx = (sx * hw + hw) as usize;
    let sy = (-sy * hh + hh) as usize;

    if sx < buff.size[0] && sy < buff.size[1] {
        buff.items[buff.linearize([sx, sy])] = 0xffffffff;
    }
}

fn render_scene(camera: &Camera, voxels: &VoxelList, buff: &mut Buffer<u32, 2>) {
    let focal = 1.0 / (camera.fov.to_radians() / 2.0).tan();
    let [hw, hh] = buff.size.map(|val| (val as f32) / 2.0);

    for i in 0..buff.size[1] {
        for j in 0..buff.size[0] {
            let [dx, dy] = [j, i].map(|val| val as f32 + 0.5);

            let ndcx = (dx - hw) / hw * camera.asp_ratio;
            let ndcy = (dy - hh) / hh;

            let vec = camera.fvec * focal + ndcx * camera.rvec + ndcy * camera.uvec;
            let ray = Ray { orig: camera.pos, dir: vec.normalize() };

            if let Some(hit) = voxels.march(ray, VSIZE as f32) {
                buff[[j, i]] = pack_color(hit.color);
            }
        }
    }

    fn pack_color(color: glam::U8Vec3) -> u32 {
        let [r, g, b] = color.to_array();
        0xff_u32 << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
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

    fn set(&mut self, location: glam::USizeVec3, color: glam::U8Vec3) {
        self.presence[location.to_array()] = true;
        self.colors[location.to_array()] = color;
    }
}
