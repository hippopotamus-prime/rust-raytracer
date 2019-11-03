use crate::vector_math::Point;
use crate::vector_math::Vector;
use crate::vector_math::PointNormal;
use crate::intersect::Intersect;
use crate::intersect::IntersectResult;


pub struct Polygon {
    pub vertices: Vec<PointNormal>
}

impl Intersect for Polygon {
    fn intersect(&self, src: &Point, ray: &Vector, near: f32) ->
            Option<IntersectResult> {
        None
    }
}
