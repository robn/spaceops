use cgmath::Point3;

use crate::wrapped_mesh::WrappedMesh;

#[derive(Clone,Debug)]
pub struct PositionAnimator {
    from:     Point3<f32>,
    to:       Point3<f32>,
    duration: f32,

    start: f64,
}

impl PositionAnimator {
    pub fn new(from: Point3<f32>, to: Point3<f32>, duration: f32) -> PositionAnimator {
        PositionAnimator {
            from:     from,
            to:       to,
            duration: duration,

            start: time::precise_time_s(),
        }
    }

    pub fn next_position(&mut self) -> Point3<f32> {
        let scale = num::clamp((time::precise_time_s() - self.start) as f32 / self.duration, 0.0, 1.0);
        self.from - ((self.to - self.from) * scale)
    }
}
