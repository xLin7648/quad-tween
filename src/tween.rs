use glam::{Vec2, Vec3};

use crate::lerp::*;
use crate::ease::*;

pub trait DefaultValue {
    fn zero() -> Self;
}

impl DefaultValue for Vec2 {
    fn zero() -> Self {
        Vec2::ZERO
    }
}

impl DefaultValue for Vec3 {
    fn zero() -> Self {
        Vec3::ZERO
    }
}

pub struct Tween<T>
where
    T: Lerp<Output = T> + DefaultValue + Copy
{
    pub timer: f32,
    pub total_time: f32,

    pub ease: Ease,
    pub is_loop: bool,
    pub result: T,
    pub points: Vec<T>,
}

impl<T> Tween<T> 
where
    T: Lerp<Output = T> + DefaultValue + Copy
{
    pub fn new(ease: Ease, is_loop: bool, total_time: f32, points: Vec<T>) -> Self {
        Self {
            timer: 0f32,
            total_time,
            is_loop,
            points,
            ease,
            result: DefaultValue::zero() // 通过泛型类型的默认值来代替具体类型的默认值
        }
    }

    pub fn update(&mut self, frame_time: f32, work: impl Fn(&mut Tween<T>)) {
        work(self);
        
        if self.timer < self.total_time {
            self.timer += frame_time;
            let t = f32::clamp(self.timer / self.total_time, 0., 1.);

            // 根据插值移动物体
            self.result = self.bezier(easeing(&self.ease, t));

        } else if self.is_loop {
            // 如果动画已经结束，重置计时器
            self.timer = 0.;
        }
    }

    // 定义一个泛型函数，实现贝塞尔曲线计算
    fn bezier(&self, t: f32) -> T
    {
        let mut result = self.points[0];
        for idx in 1..self.points.len() {
            result = result.lerp(self.points[idx], t);
        }
        result
    }

    pub fn in_progress(&self) -> bool {
        self.is_loop || self.timer < self.total_time
    }
}