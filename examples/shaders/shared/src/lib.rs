//! Ported to Rust from <https://github.com/Tw1ddle/Sky-Shader/blob/master/src/shaders/glsl/sky.fragment>

#![cfg_attr(target_arch = "spirv", no_std, feature(lang_items))]

use core::f32::consts::PI;
use glam::{vec2, vec3, Vec2, Vec3};

pub use spirv_std::glam;

// Note: This cfg is incorrect on its surface, it really should be "are we compiling with std", but
// we tie #[no_std] above to the same condition, so it's fine.
use bytemuck::{Pod, Zeroable};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,

    pub cursor_x: f32,
    pub cursor_y: f32,
    pub drag_start_x: f32,
    pub drag_start_y: f32,
    pub drag_end_x: f32,
    pub drag_end_y: f32,

    /// Bit mask of the pressed buttons (0 = Left, 1 = Middle, 2 = Right).
    pub mouse_button_pressed: u32,

    /// The last time each mouse button (Left, Middle or Right) was pressed,
    /// or `f32::NEG_INFINITY` for buttons which haven't been pressed yet.
    ///
    /// If this is the first frame after the press of some button, that button's
    /// entry in `mouse_button_press_time` will exactly equal `time`.
    pub mouse_button_press_time: [f32; 3],
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Vec2Homebrew {
    // has to have 4 floats to satisfy the alignment requirements??? God save me
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Vec4Homebrew {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Circle {
    pub pos: Vec2Homebrew,
    pub color: Vec4Homebrew,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct CirclesToDisplay {
    pub circles: [Circle; NUM_CIRCLES as usize],
}

pub fn saturate(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

pub fn pow(v: Vec3, power: f32) -> Vec3 {
    vec3(v.x.powf(power), v.y.powf(power), v.z.powf(power))
}

pub fn exp(v: Vec3) -> Vec3 {
    vec3(v.x.exp(), v.y.exp(), v.z.exp())
}

/// Based on: <https://seblagarde.wordpress.com/2014/12/01/inverse-trigonometric-functions-gpu-optimization-for-amd-gcn-architecture/>
pub fn acos_approx(v: f32) -> f32 {
    let x = v.abs();
    let mut res = -0.155972 * x + 1.56467; // p(x)
    res *= (1.0f32 - x).sqrt();

    if v >= 0.0 {
        res
    } else {
        PI - res
    }
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    // Scale, bias and saturate x to 0..1 range
    let x = saturate((x - edge0) / (edge1 - edge0));
    // Evaluate polynomial
    x * x * (3.0 - 2.0 * x)
}

const NUM_CIRCLES: u32 = 32;
const RESOLUTION: [f32; 2] = [1920.0, 1080.0];

pub fn generate_random_vecs() -> [Vec2Homebrew; NUM_CIRCLES as usize] {
    // causes a pointer error?
    let mut random_nums: [Vec2Homebrew; NUM_CIRCLES as usize] = [Vec2Homebrew {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    }; NUM_CIRCLES as usize];
    for i in 0..NUM_CIRCLES {
        random_nums[i as usize].x =
            random(vec2(i as f32 * 100.0, (i as i32 * 2 - 1245) as f32)).abs() * RESOLUTION[0];
        random_nums[i as usize].y =
            random(vec2(i as f32 * -100.0 + 50.0, (i as i32 * 5 + 1259) as f32)).abs()
                * RESOLUTION[1];
    }
    return random_nums;
}

pub fn generate_circles() -> CirclesToDisplay {
    let mut circles: [Circle; NUM_CIRCLES as usize] = [Circle {
        pos: Vec2Homebrew {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        color: Vec4Homebrew {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
    }; NUM_CIRCLES as usize];

    let circle_positions = generate_random_vecs();
    for i in 0..NUM_CIRCLES {
        circles[i as usize].pos = circle_positions[i as usize];
        circles[i as usize].color = Vec4Homebrew {
            x: random(vec2((i * 100) as f32, (i + 2 * 5) as f32)),
            y: random(vec2((i * 17) as f32, (i + 5 * 8) as f32)),
            z: random(vec2((i * 25) as f32, (i + 8 * 2) as f32)),
            w: 1.0,
        };
    }
    return CirclesToDisplay { circles: circles };
}

/*generates a number between 1 and -1 */
pub fn random(uv: Vec2) -> f32 {
    return f32::fract(f32::sin(uv.x * 12.9898 + uv.y * 78.233) * 43758.5453123);
}
