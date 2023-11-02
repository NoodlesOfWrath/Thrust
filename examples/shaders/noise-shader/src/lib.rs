#![cfg_attr(target_arch = "spirv", no_std)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
#![deny(warnings)]

use shared::glam::{vec2, vec4, Vec2, Vec4};
use spirv_std::num_traits::Float;
use spirv_std::spirv;
const RESOLUTION: [f32; 2] = [1920.0, 1080.0]; // temporary solution

pub fn random(uv: Vec2) -> f32 {
    return Float::fract(Float::sin(uv.x * 12.9898 + uv.y * 78.233) * 43758.5453123);
}

#[spirv(fragment)]
pub fn main_fs(#[spirv(frag_coord)] in_frag_coord: Vec4, out: &mut Vec4) {
    let random_1 = random(vec2(in_frag_coord.x * 10.0, in_frag_coord.y * 10.0));
    let random_2 = random(vec2(
        in_frag_coord.x * 10.0 - 5762.0,
        in_frag_coord.y * 10.0 - 50205.0,
    ));
    let random_3 = random(vec2(
        in_frag_coord.x * 10.0 + 6574.0,
        in_frag_coord.y * 10.0 - 1000.0,
    ));
    *out = vec4(random_1, random_2, random_3, 1.0);
}

#[spirv(vertex)]
pub fn main_vs(#[spirv(vertex_index)] vert_idx: i32, #[spirv(position)] builtin_pos: &mut Vec4) {
    // Create a "full screen triangle" by mapping the vertex index.
    // ported from https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
    let uv = vec2(((vert_idx << 1) & 2) as f32, (vert_idx & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *builtin_pos = pos.extend(0.0).extend(1.0);
}
