use minifb::Key;

#[derive(Debug, Default)]
pub struct Camera {
    pub pos: glam::Vec3A,
    pub fvec: glam::Vec3A,
    pub rvec: glam::Vec3A,
    pub uvec: glam::Vec3A,
    pub asp_ratio: f32,
    pub fov: f32,
    pub look_speed: f32,
    pub move_speed: f32,
}

impl Camera {
    pub fn build(ar: f32, fov: f32) -> Self {
        Self {
            fvec: -glam::Vec3A::Z,
            rvec: glam::Vec3A::X,
            uvec: glam::Vec3A::Y,
            asp_ratio: ar,
            fov,
            ..Default::default()
        }
    }

    pub fn update_rotation(&mut self, rot: glam::Vec3A) {
        let updated = glam::Mat3A::from_rotation_z(rot.z * -self.look_speed)
            .mul_mat3(&glam::Mat3A::from_rotation_y(rot.y * -self.look_speed))
            .mul_mat3(&glam::Mat3A::from_rotation_x(rot.x * -self.look_speed))
            .mul_mat3(&glam::Mat3A::from_cols(self.rvec, self.uvec, self.fvec));
        self.fvec = updated.z_axis;
        self.uvec = updated.y_axis;
        self.rvec = updated.x_axis;
    }

    pub fn update_position(&mut self, translation: glam::Vec3A) {
        let translation = translation.normalize_or_zero();
        self.pos -= self.fvec * translation.z * -self.move_speed;
        self.pos += self.uvec * translation.y * -self.move_speed;
        self.pos += self.rvec * translation.x * -self.move_speed;
    }
}

impl std::fmt::Display for Camera {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "f: ({:.2}, {:.2}, {:.2})", self.fvec.x, self.fvec.y, self.fvec.z)?;
        write!(fmt, "r: ({:.2}, {:.2}, {:.2})", self.rvec.x, self.rvec.y, self.rvec.z)?;
        write!(fmt, "u: ({:.2}, {:.2}, {:.2})", self.uvec.x, self.uvec.y, self.uvec.z)?;
        writeln!(fmt, "{}", self.pos)?;
        Ok(())
    }
}

pub fn minifb_input_cb(window: &minifb::Window, camera: &mut Camera) {
    let mut rotation = glam::Vec3A::ZERO;
    let mut translation = glam::Vec3A::ZERO;
    if window.is_key_down(Key::Up) {
        rotation.x += 1.0;
    }
    if window.is_key_down(Key::Down) {
        rotation.x -= 1.0;
    }
    if window.is_key_down(Key::Left) {
        rotation.y += 1.0;
    }
    if window.is_key_down(Key::Right) {
        rotation.y -= 1.0;
    }
    if window.is_key_down(Key::W) {
        translation.z += 1.0;
    }
    if window.is_key_down(Key::S) {
        translation.z -= 1.0;
    }
    if window.is_key_down(Key::A) {
        translation.x -= 1.0;
    }
    if window.is_key_down(Key::D) {
        translation.x += 1.0;
    }
    camera.update_rotation(rotation);
    camera.update_position(translation);
}
