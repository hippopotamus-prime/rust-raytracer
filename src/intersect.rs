use crate::vector_math::Point;
use crate::vector_math::Vector;

pub struct IntersectResult {
    pub normal: Vector,
    pub dist: f32
}

pub trait Intersect {
    fn intersect(&self, src: &Point, ray: &Vector, near: f32) ->
            Option<IntersectResult>;
}
