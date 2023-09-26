#![no_std]

use spirv_std::spirv;
use spirv_std::glam::{Vec4, Vec3, Vec2};

pub struct CustomMaterial {
    pub color: Vec4,
}

#[spirv(fragment)]
#[allow(unused_variables)]
pub fn main_fs(
    #[spirv(position)] position: Vec4,
    #[spirv(uniform, descriptor_set=1, binding=0)] material: &CustomMaterial,
    world_position: Vec4,
    world_normal: Vec3,
    uv: Vec2,
    world_tangent: Vec4,
    color: Vec4,
    output: &mut Vec4
) {
    let consume = 1.0 + (world_position.x + world_normal.x + uv.x).clamp(0.0, 0.0);
    *output = material.color * consume
}
