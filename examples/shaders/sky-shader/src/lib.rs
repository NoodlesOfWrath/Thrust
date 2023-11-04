#![cfg_attr(target_arch = "spirv", no_std)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
#![deny(warnings)]

use shared::glam::{vec2, vec4, Vec2, Vec4};
use shared::{Circle, CirclesToDisplay, Vec2Homebrew};
use spirv_std::spirv;
const NUM_CIRCLES: u32 = 32;

pub struct CircleWrapper {
    pub circles: CirclesToDisplay,
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] in_frag_coord: Vec4,
    out: &mut Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] circle_wrapper: &CircleWrapper,
) {
    let circles = circle_wrapper.circles.circles;
    let mut final_out = vec4(0.0, 0.0, 0.0, 0.0);
    let frag_coords = vec2(in_frag_coord.x, in_frag_coord.y);
    for i in 0..NUM_CIRCLES {
        let circle_pass = render_circle(frag_coords, circles[i as usize]);
        final_out = final_out + circle_pass;
    }
    *out = final_out;
}

pub fn render_circle(frag_coord: Vec2, circle: Circle) -> Vec4 {
    let center = Vec2Homebrew {
        x: circle.pos.x * 0.001,
        y: circle.pos.y * 0.001,
        z: 0.0,
        w: 0.0,
    };
    let radius_squared = 100.0;
    let dx = frag_coord.x - center.x;
    let dy = frag_coord.y - center.y;

    if (dx * dx) + (dy * dy) < radius_squared {
        return vec4(
            circle.color.x,
            circle.color.y,
            circle.color.z,
            circle.color.w,
        );
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
