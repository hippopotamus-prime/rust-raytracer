use crate::vector_math;
use crate::vector_math::{Point, Vector};
use crate::shape::{Shape, IntersectResult, BoundingBox};


pub struct Sphere {
    pub center: Point,
    pub radius: f32
}

impl Shape for Sphere {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            corner: Point {
                x: self.center.x - self.radius,
                y: self.center.y - self.radius,
                z: self.center.z - self.radius
            },
            extent: Vector {
                dx: self.radius * 2.0,
                dy: self.radius * 2.0,
                dz: self.radius * 2.0
            }
        }
    }

    fn intersect(&self, src: &Point, ray: &Vector, near: f32) ->
            Option<IntersectResult> {
        // Find a solution to the equations:
        //  p = src + t * ray           (ray equation)
        //  ||p - center|| = radius     (edge of the sphere)
        // where p is the intersection point and t is its distance
        // along the ray.
        //
        // So...
        // ||src + t * ray - center|| = radius
        // (src - center + t * ray) . (src - center + t * ray) = radius^2
        // (src - center) . (src - center) + (t * ray) . (t * ray) +
        //          2 * (src - center) . (t * ray) = radius^2
        // [ray . ray] t^2 +
        //          2 [(src - center) . ray] t +
        //          (src - center) . (src - center) - radius^2 = 0
        //
        // Solve for t using the quadratic equation, with:
        // a = ray . ray
        // b = 2 [(src - center) . ray]
        // c = (src - center) . (src - center) - radius^2

        let sc = src - &self.center;

        let a = vector_math::dot(ray, ray);
        let b = 2.0 * vector_math::dot(ray, &sc);
        let c = vector_math::dot(&sc, &sc) - self.radius * self.radius;

        let b2m4ac = b * b - 4.0 * a * c;
        if b2m4ac >= 0.0 {
            let sq = b2m4ac.sqrt();

            // There are two solutions since the ray intersects the sphere
            // twice - we'll use the shorter one unless it's behind the near
            // plane.
            let mut t = (-b - sq) / (2.0 * a);
            if t < near {
                t = (-b + sq) / (2.0 * a);
            }

            if t >= near {
                // The surface normal has the same direction as the
                // intersection point from the center.
                let normal = (src + t * ray - &self.center).normalized();
                return Some(IntersectResult {
                    normal: normal,
                    dist: t
                });
            }
        }

        None
    }
}