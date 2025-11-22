mod buffer;
mod camera;
mod raymarch;
mod tree;

use minifb::Key;

use crate::{buffer::Buffer, camera::Camera, raymarch::RayMarch};

const WIDTH: usize = 400;
const HEIGHT: usize = 300;
const VSIZE: usize = 128;

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
    camera.pos = glam::Vec3A::new(32.0, 8.0, 32.0);
    let mut vox = raymarch::VoxelList::new(glam::USizeVec3::new(VSIZE, VSIZE, VSIZE));
    setup_debug_scene(&mut vox);
    let test_point = glam::Vec3A::new(0.0, 0.0, -3.0);

    loop {
        if window.is_key_down(Key::Escape) {
            break;
        }

        buffer.fill(0);
        camera::minifb_input_cb(&window, &mut camera);
        debug_render_scene(&camera, &vox, &mut buffer);
        render_test_point(&camera, test_point, &mut buffer);
        window.update_with_buffer(&buffer.items, buffer.size[0], buffer.size[1]).unwrap();
    }
}

fn setup_debug_scene(vox: &mut raymarch::VoxelList) {
    for i in 0..VSIZE {
        for j in 0..VSIZE {
            vox.set(glam::USizeVec3::new(i, 0, j), glam::U8Vec3::new(40, 255, 40));
        }
    }
    for i in 0..4 {
        vox.set(glam::USizeVec3::new(1, i, 5), glam::U8Vec3::new(0, 255, 255));
        vox.set(glam::USizeVec3::new(4, i, 1), glam::U8Vec3::new(0, 255, 255));
        vox.set(glam::USizeVec3::new(0, i, 2), glam::U8Vec3::new(0, 255, 255));
    }
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                vox.set(glam::USizeVec3::new(10 + i, 10 + j, 10 + k), glam::U8Vec3::new(255, 10, 10));
            }
        }
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

fn debug_render_scene(camera: &Camera, voxels: &raymarch::VoxelList, buff: &mut Buffer<u32, 2>) {
    let focal = 1.0 / (camera.fov.to_radians() / 2.0).tan();
    let [hw, hh] = buff.size.map(|val| (val as f32) / 2.0);

    for i in 0..buff.size[1] {
        for j in 0..buff.size[0] {
            let [dx, dy] = [j, i].map(|val| val as f32 + 0.5);

            let ndcx = (dx - hw) / hw;
            let ndcy = (dy - hh) / hh;

            debug_assert!((-1.0..1.0).contains(&ndcx));
            debug_assert!((-1.0..1.0).contains(&ndcy));

            let dx = camera.rvec * -ndcx * focal * camera.asp_ratio;
            let dy = camera.uvec * ndcy * focal;

            let ray = raymarch::Ray {
                orig: camera.pos,
                dir: (camera.fvec + dx + dy).normalize(),
            };

            if let Some(hit) = voxels.march(ray, 64.0) {
                buff[[j, i]] = pack_color(hit.color);
            }
        }
    }

    fn pack_color(color: glam::U8Vec3) -> u32 {
        let [r, g, b] = color.to_array();
        0xff_u32 << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
    }
}
