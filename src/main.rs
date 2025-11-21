use std::fmt::Display;

use minifb::Key;

const WIDTH: usize = 150;
const HEIGHT: usize = 100;

fn main() {
    let mut window = minifb::Window::new(
        "voxel",
        WIDTH,
        HEIGHT,
        minifb::WindowOptions { scale: minifb::Scale::X4, ..Default::default() },
    )
    .unwrap();
    let mut camera = Camera::build(WIDTH as f32 / HEIGHT as f32, 90.0);
    camera.sense = 0.01;
    camera.speed = 0.01;;
    let mut buffer = Buffer::build(0u32, [WIDTH, HEIGHT]);
    let test_point = glam::Vec3A::new(0.0, 0.0, -10.0);

    loop {
        if window.is_key_down(Key::Escape) {
            break;
        }
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
            translation.x +=  1.0;
        }
        camera.update_rotation(rotation);
        camera.update_position(translation);

        buffer.fill(0);
        render_test_point(&camera, test_point, &mut buffer);

        println!("{}", &camera);

        window.update_with_buffer(&buffer.items, buffer.size[0], buffer.size[1]).unwrap();
    }
}

fn render_test_point(camera: &Camera, point: glam::Vec3A, buff: &mut Buffer<u32, 2>) {
    let rel = point - camera.pos;

    let vx = camera.rv.dot(rel);
    let vy = camera.uv.dot(rel);
    let vz = camera.fv.dot(rel);

    if vz < 0.0 {
        return;
    }
    let focal = 1.0 / (camera.fov.to_radians() / 2.0).tan();

    let sx = vx / -vz * focal * camera.ar;
    let sy = vy / -vz* focal;

    let [hw, hh] = buff.size.map(|val| (val / 2) as f32);

    let sx = (sx * hw + hw) as usize;
    let sy = (-sy * hh + hh) as usize;

    if sx < buff.size[0] && sy < buff.size[1] {
        buff.items[buff.linearize([sx, sy])] = 0xffffffff;
    }
}

#[derive(Debug, Default)]
struct Camera {
    pos: glam::Vec3A,
    fv: glam::Vec3A,
    rv: glam::Vec3A,
    uv: glam::Vec3A,
    wuv: glam::Vec3A,
    ar: f32,
    fov: f32,
    sense: f32,
    speed: f32,
}

impl Display for Camera {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "f: ({:.2}, {:.2}, {:.2})", self.fv.x, self.fv.y, self.fv.z)?;
        write!(fmt, "r: ({:.2}, {:.2}, {:.2})", self.rv.x, self.rv.y, self.rv.z)?;
        write!(fmt, "u: ({:.2}, {:.2}, {:.2})", self.uv.x, self.uv.y, self.uv.z)?;
        writeln!(fmt, "{}", self.pos)?;
        Ok(())
    }
}

impl Camera {
    fn build(ar: f32, fov: f32) -> Self {
        Self {
            fv: -glam::Vec3A::Z,
            rv: glam::Vec3A::X,
            uv: glam::Vec3A::Y,
            wuv: glam::Vec3A::Y,
            ar,
            fov,
            ..Default::default()
        }
    }

    fn update_rotation(&mut self, rot: glam::Vec3A) {
        let updated = glam::Mat3A::from_rotation_z(rot.z * self.sense)
            * glam::Mat3A::from_rotation_y(rot.y * self.sense)
            * glam::Mat3A::from_rotation_x(rot.x * self.sense)
            * glam::Mat3A::from_cols(self.rv, self.wuv, self.fv);
        self.fv = updated.z_axis;
        self.uv = updated.y_axis;
        self.rv = updated.x_axis;
    }

    fn update_position(&mut self, translation: glam::Vec3A) {
        self.pos -= self.fv * translation.z * self.speed;
        self.pos += self.uv * translation.y * self.speed;
        self.pos += self.rv * translation.x * self.speed;
    }
}

struct Buffer<T, const N: usize> {
    items: Box<[T]>,
    size: [usize; N],
}

impl<const N: usize, T> Buffer<T, N> {
    fn build(fill: T, size: [usize; N]) -> Self
    where
        T: Clone + Copy,
    {
        Self { items: vec![fill; size.iter().product()].into(), size }
    }

    fn linearize(&self, indices: [usize; N]) -> usize {
        let mut index = 0;
        let mut stride = 1;
        for idx in (0..N).rev() {
            index += indices[idx] * stride;
            stride *= self.size[idx];
        }
        index
    }

    fn fill(&mut self, fill: T)
    where
        T: Clone + Copy,
    {
        self.items.iter_mut().for_each(|item| *item = fill);
    }
}
