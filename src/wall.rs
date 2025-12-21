use crate::{buffer, camera};

#[derive(Default, Clone, Copy)]
pub struct Wall {
    pub p1: glam::IVec2,
    pub p2: glam::IVec2,
    pub h: i32,
    pub col: u32,
}

pub fn render_wall(buff: &mut buffer::Buffer, cam: &camera::Camera, wall: &Wall) {
    let focal = 1.0 / (cam.fov.to_radians() / 2.0).tan();
    let [hw, hh] = buff.size.map(|val| (val / 2) as f32);

    let p1 = to_camera(wall.p1, cam);
    let p2 = to_camera(wall.p2, cam);

    let x1 = to_screenx(p1, hw, focal);
    let x2 = to_screenx(p2, hw, focal);

    let h1 = to_screeny(wall.h, p1, hh, focal);
    let h2 = to_screeny(wall.h, p2, hh, focal);

    let dhdx = (h2 - h1) / (x2 - x1).max(1.0f32);

    for x in (x1 as i32).clamp(0, buff.size[0] as i32 - 1)..(x2 as i32).clamp(0, buff.size[0] as i32 - 1) {
        let dh = h1 + (x as f32 - x1) * dhdx;

        let y1 = (hh - dh / 2.0) as i32;
        let y2 = (hh + dh / 2.0) as i32;

        fill_column(buff, wall, x, y1, y2);
    }
}

fn fill_column(buff: &mut buffer::Buffer, wall: &Wall, x: i32, y1: i32, y2: i32) {
    for y in y1.clamp(0, buff.size[1] as i32 - 1)..y2.clamp(0, buff.size[1] as i32 - 1) {
        _ = buff.try_set(wall.col, [x as usize, y as usize]);
    }
}

#[inline(always)]
fn to_camera(pos: glam::IVec2, cam: &camera::Camera) -> glam::Vec2 {
    let rel = pos.as_vec2() - cam.pos;
    glam::vec2(rel.dot(cam.rvec), rel.dot(cam.fvec))
}

#[inline(always)]
fn to_screenx(pos: glam::Vec2, hw: f32, focal: f32) -> f32 {
    hw + pos.x / pos.y * hw * focal
}

#[inline(always)]
fn to_screeny(height: i32, pos: glam::Vec2, hh: f32, focal: f32) -> f32 {
    height as f32 / pos.y * hh * focal
}
