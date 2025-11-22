use crate::buffer::Buffer;

pub struct Ray {
    pub orig: glam::Vec3A,
    pub dir: glam::Vec3A,
}

#[derive(Default)]
pub struct RayHit {
    pub color: glam::U8Vec3,
    pub normal: glam::Vec3A,
    pub time: f32,
}

pub trait RayMarch {
    fn march(&self, ray: Ray, tmax: f32) -> Option<RayHit>;
}

pub struct VoxelList {
    pub presence: Buffer<bool, 3>,
    pub colors: Buffer<glam::U8Vec3, 3>,
}

impl VoxelList {
    pub fn new(size: glam::USizeVec3) -> Self {
        Self {
            presence: Buffer::build(false, size.to_array()),
            colors: Buffer::build(glam::U8Vec3::default(), size.to_array()),
        }
    }

    pub fn set(&mut self, location: glam::USizeVec3, color: glam::U8Vec3) {
        self.presence[location.to_array()] = true;
        self.colors[location.to_array()] = color;
    }
}

impl RayMarch for VoxelList {
    fn march(&self, ray: Ray, tmax: f32) -> Option<RayHit> {
        let Ray { orig, dir } = ray;
        debug_assert!(dir.is_normalized());
        debug_assert!(tmax > 0.0);

        let mut voxel = orig.floor().as_ivec3();
        let step = dir.signum().as_ivec3();
        let inv_dir = 1.0 / dir.abs();
        let fract = orig.fract();
        let boundary_distance = glam::Vec3A::new(
            if dir.x < 0.0 { fract.x } else { 1.0 - fract.x },
            if dir.y < 0.0 { fract.y } else { 1.0 - fract.y },
            if dir.z < 0.0 { fract.z } else { 1.0 - fract.z },
        );
        let mut tvec = boundary_distance * inv_dir;

        loop {
            let idx = voxel.as_usizevec3().to_array();
            if self.presence.surrounds(idx) && self.presence[idx] {
                return Some(RayHit { color: self.colors[idx], ..Default::default() });
            }

            let (axis, next_t) = {
                let mut axis = Axis::X;
                let mut next_t = tvec.x;
                if tvec.y < next_t {
                    axis = Axis::Y;
                    next_t = tvec.y;
                }
                if tvec.z < next_t {
                    axis = Axis::Z;
                    next_t = tvec.z;
                }
                (axis, next_t)
            };

            if next_t > tmax {
                break;
            }

            match axis {
                | Axis::X => {
                    voxel.x += step.x;
                    tvec.x += inv_dir.x;
                }
                | Axis::Y => {
                    voxel.y += step.y;
                    tvec.y += inv_dir.y;
                }
                | Axis::Z => {
                    voxel.z += step.z;
                    tvec.z += inv_dir.z;
                }
            }
        }

        return None;

        enum Axis {
            X,
            Y,
            Z,
        }
    }
}
