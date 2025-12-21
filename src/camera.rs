use std::fmt;

use glam::Vec3Swizzles;
use minifb::Key;

#[derive(Debug, Default)]
pub struct Camera {
    pub pitch: f32,
    pub yaw: f32,

    pub fvec: glam::Vec2,
    pub rvec: glam::Vec2,
    pub pos: glam::Vec2,

    pub ar: f32,
    pub fov: f32,

    pub look_speed: f32,
    pub move_speed: f32,
}

impl Camera {
    pub fn build(ar: f32, fov: f32) -> Self {
        Self { ar, fov, ..Default::default() }
    }

    pub fn update_rotation(&mut self, pitch: f32, yaw: f32) {
        self.pitch += pitch;
        self.yaw += yaw;

        self.pitch = self.pitch.clamp(-89.9f32, 89.9f32);
        self.yaw = self.yaw.rem_euclid(360.0f32);

        self.fvec = glam::Mat2::from_angle(self.yaw.to_radians()) * glam::Vec2::Y;
        self.rvec = glam::vec3(self.fvec.x, self.fvec.y, 0.0f32).cross(glam::Vec3::Z).xy();
    }

    pub fn update_position(&mut self, movement: glam::Vec2) {
        self.pos += self.fvec * movement.y;
        self.pos += self.rvec * movement.x;
    }

    pub fn display(&self) {
        print!("{}", self);
    }
}

impl fmt::Display for Camera {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "f: ({:.2}, {:.2})", self.fvec.x, self.fvec.y)?;
        write!(fmt, "r: ({:.2}, {:.2})", self.rvec.x, self.rvec.y)?;
        write!(fmt, "pitch: {:.2}, yaw: {:.2}", self.pitch, self.yaw)?;
        writeln!(fmt, "\n{}", self.pos)?;
        Ok(())
    }
}

pub fn minifb_input_cb(window: &minifb::Window, camera: &mut Camera) {
    let mut translation = glam::IVec2::ZERO;
    let mut yaw = 0;
    let mut pitch = 0;
    if window.is_key_down(Key::W) {
        translation.y += 1;
    }
    if window.is_key_down(Key::S) {
        translation.y -= 1;
    }
    if window.is_key_down(Key::D) {
        translation.x += 1;
    }
    if window.is_key_down(Key::A) {
        translation.x -= 1
    }
    if window.is_key_down(Key::Right) {
        yaw -= 1;
    }
    if window.is_key_down(Key::Left) {
        yaw += 1;
    }
    if window.is_key_down(Key::Up) {
        pitch += 1;
    }
    if window.is_key_down(Key::Down) {
        pitch -= 1;
    }
    camera.update_position(translation.as_vec2());
    camera.update_rotation(pitch as f32, yaw as f32);
}
