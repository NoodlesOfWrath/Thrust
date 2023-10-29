#![cfg_attr(target_arch = "spirv", no_std)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
#![deny(warnings)]

use shared::glam::{vec2, vec4, Vec2, Vec4};
use spirv_std::num_traits::Float;
use spirv_std::spirv;
const RESOLUTION: [f32; 2] = [1920.0, 1080.0]; // temporary solution
const NUM_CIRCLES: i32 = 1000;

pub fn generate_random_vecs() -> [Vec2; NUM_CIRCLES as usize] {
    let mut random_nums: [Vec2; NUM_CIRCLES as usize] = [vec2(0.0, 0.0); 1000];
    for seed in 1254..(1254 + NUM_CIRCLES) {
        for j in 0..2 {
            random_nums[(seed - 1254) as usize][j] = random(vec2(
                j as f32 * (seed - 1254) as f32 * 100 as f32,
                seed as f32,
            )) * RESOLUTION[j];
        }
    }
    return random_nums;
}

pub fn random(uv: Vec2) -> f32 {
    return Float::fract(Float::sin(uv.x * 12.9898 + uv.y * 78.233) * 43758.5453123);
}

#[spirv(fragment)]
pub fn main_fs(#[spirv(frag_coord)] in_frag_coord: Vec4, out: &mut Vec4) {
    let random = random(vec2(in_frag_coord.x, in_frag_coord.y));
    *out = vec4(random, random, random, 1.0);
    /*let mut final_out = vec4(0.0, 0.0, 0.0, 0.0);
    let circle_positions = generate_random_vecs();
    let frag_coords = vec2(in_frag_coord.x, in_frag_coord.y);
    for i in 0..NUM_CIRCLES {
        let circle_pos = circle_positions[i as usize];
        final_out += render_circle(frag_coords, circle_pos, vec4(1.0, 1.0, 1.0, 1.0));
    }
    *out = final_out;*/
}

pub fn render_circle(frag_coord: Vec2, pos: Vec2, color: Vec4) -> Vec4 {
    let center = pos;
    let radius_squared = 1000.0;
    let dx = frag_coord.x - center.x;
    let dy = frag_coord.y - center.y;

    if (dx * dx) + (dy * dy) < radius_squared {
        return color;
    } else {
        return vec4(0.0, 0.0, 0.0, 0.0);
    }
}

#[spirv(vertex)]
pub fn main_vs(#[spirv(vertex_index)] vert_idx: i32, #[spirv(position)] builtin_pos: &mut Vec4) {
    // Create a "full screen triangle" by mapping the vertex index.
    // ported from https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
    let uv = vec2(((vert_idx << 1) & 2) as f32, (vert_idx & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *builtin_pos = pos.extend(0.0).extend(1.0);
}
