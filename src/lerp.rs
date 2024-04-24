use glam::{vec2, vec3, Vec2, Vec3};

pub fn i32_lerp(start: i32, end: i32, t: i32) -> i32 {
    start + (end - start) * t
}

pub fn i64_lerp(start: i64, end: i64, t: i64) -> i64 {
    start + (end - start) * t
}

pub fn f32_lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

pub fn f64_lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

pub fn vec2_lerp(start: Vec2, end: Vec2, t: f32) -> Vec2 {
    vec2(
        start.x + (end.x - start.x) * t,
        start.y + (end.y - start.y) * t,
    )
}

pub fn vec3_lerp(start: Vec3, end: Vec3, t: f32) -> Vec3 {
    vec3(
        start.x + (end.x - start.x) * t,
        start.y + (end.y - start.y) * t,
        start.z + (end.z - start.z) * t
    )
}

// 定义一个泛型 trait 用于向量插值
pub trait Lerp<Rhs = Self> {
    type Output;
    fn lerp(self, rhs: Rhs, t: f32) -> Self::Output;
}

// 实现 Vec2 的线性插值
impl Lerp for Vec2 {
    type Output = Vec2;
    fn lerp(self, rhs: Vec2, t: f32) -> Self::Output {
        self * (1.0 - t) + rhs * t
    }
}

// 实现 Vec3 的线性插值
impl Lerp for Vec3 {
    type Output = Vec3;
    fn lerp(self, rhs: Vec3, t: f32) -> Self::Output {
        self * (1.0 - t) + rhs * t
    }
}