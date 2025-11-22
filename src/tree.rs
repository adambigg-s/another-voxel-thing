use crate::raymarch;

pub struct AaBb {
    low: glam::USizeVec3,
    high: glam::USizeVec3,
}

pub struct TreeNode {
    pub bounds: AaBb,
    pub leaves: Option<usize>,
    pub dataptr: Option<usize>,
}

pub struct Tree8 {
    pub nodes: Vec<TreeNode>,
    pub nodeptr: Vec<[usize; Self::COUNT]>,
}

impl Tree8 {
    pub const ROOT: usize = 0;
    pub const COUNT: usize = 8;
}

impl raymarch::RayMarch for Tree8 {
    fn march(&self, ray: raymarch::Ray, tmax: f32) -> Option<raymarch::RayHit> {
        todo!()
    }
}
