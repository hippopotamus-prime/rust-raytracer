use crate::vector_math;
use crate::vector_math::{Point, Vector};
use crate::shape::{Shape, IntersectResult, BoundingBox};

// Generalized cone & cylinder - cones have apex_radius 0, cylinders have
// apex_radius = base_radius, cone frustums are somewhere in between.
pub struct Cone {
    pub base: Point,
    pub apex: Point,
    pub base_radius: f32,
    pub apex_radius: f32
}

impl Shape for Cone {
    fn bounding_box(&self) -> BoundingBox {
        // The box surrounds the central line of the cone, extended out by
        // the x/y/z components of the base and apex.  To get a tight bound,
        // we'll project each axis onto the plane of the base/apex disc,
        // scale by the base/apex radius, then take the axial component of the
        // result.
        //
        // For example for x and the base, that works out to:
        // n = apex - base, normalized
        // [base_radius * ( {1, 0, 0} - n * ({1, 0, 0} dot n) )].x
        // = [base_radius * ( {1, 0, 0} - n * n.dx )].x
        // = [base_radius * (1 - n.dx * n.dx)]
        //
        // (For the apex, n is reversed, but the math makes it not matter.)

        let n = (&self.apex - &self.base).normalized();
        let base_x_extra = self.base_radius * (1.0 - n.dx * n.dx);
        let base_y_extra = self.base_radius * (1.0 - n.dy * n.dy);
        let base_z_extra = self.base_radius * (1.0 - n.dz * n.dz);
        let apex_x_extra = self.apex_radius * (1.0 - n.dx * n.dx);
        let apex_y_extra = self.apex_radius * (1.0 - n.dy * n.dy);
        let apex_z_extra = self.apex_radius * (1.0 - n.dz * n.dz);

        let (corner_x, extent_x) = if self.base.x < self.apex.x {
            (self.base.x - base_x_extra,
                self.apex.x - self.base.x + base_x_extra + apex_x_extra)
        } else {
            (self.apex.x - apex_x_extra,
                self.base.x - self.apex.x + base_x_extra + apex_x_extra)
        };

        let (corner_y, extent_y) = if self.base.y < self.apex.y {
            (self.base.y - base_y_extra,
                self.apex.y - self.base.y + base_y_extra + apex_y_extra)
        } else {
            (self.apex.y - apex_y_extra,
                self.base.y - self.apex.y + base_y_extra + apex_y_extra)
        };

        let (corner_z, extent_z) = if self.base.z < self.apex.z {
            (self.base.z - base_z_extra,
                self.apex.z - self.base.z + base_z_extra + apex_z_extra)
        } else {
            (self.apex.z - apex_z_extra,
                self.base.z - self.apex.z + base_z_extra + apex_z_extra)
        };

        BoundingBox {
            corner: Point {
                x: corner_x,
                y: corner_y,
                z: corner_z
            },
            extent: Vector {
                dx: extent_x,
                dy: extent_y,
                dz: extent_z
            }
        }
    }

    fn intersect(&self, src: &Point, ray: &Vector, near: f32) ->
            Option<IntersectResult> {
        // Notes copied from the C++ version...
        //
        // It's a good five pages of derivations to get
        // something that works for both cone and cylinder...
        // The basic idea here is that the general equation for
        // a cone/cylinder aligned to the w-axis can be written as:
        //  r = B - A * w
        // Here B is the base height, and A is B minus the apex
        // height, divided by the length of the cone/cylinder.
        // So at the base (w = 0), r = B, and at the apex
        // (w = ||axis||), r = A.

        // First we're going to want to change to a new basis,
        // with one axis (w) going up the center of the cylinder,
        // and the others (u and v) in the plane of the base.

        let base_to_apex = &self.apex - &self.base;
        let w = base_to_apex.normalized();

        // To get the next basis vector, we can cross w with
        // anything that is not a multiple of w.  To avoid this
        // we'll pick the axis corresponding to w's smallest
        // component.

        let shortest_w_component =
            if w.dx.abs() < w.dy.abs() && w.dx.abs() < w.dz.abs() {
                Vector {dx: 1.0, dy: 0.0, dz: 0.0}
            } else if w.dy.abs() < w.dz.abs() {
                Vector {dx: 0.0, dy: 1.0, dz: 0.0}
            } else {
                Vector {dx: 0.0, dy: 0.0, dz: 1.0}
            };

        // Note: u & v will already be normalized
        let u = vector_math::cross(&w, &shortest_w_component);
        let v = vector_math::cross(&w, &u);

        // Now we need to convert the ray and src to the new
        // basis, which has its origin at the base.

        let base_to_src = src - &self.base;
        let src_uvw = Vector {
            dx: vector_math::dot(&base_to_src, &u),
            dy: vector_math::dot(&base_to_src, &v),
            dz: vector_math::dot(&base_to_src, &w)
        };

        let ray_uvw = Vector {
            dx: vector_math::dot(&ray, &u),
            dy: vector_math::dot(&ray, &v),
            dz: vector_math::dot(&ray, &w)
        };

        // Now we can do the actual computation, which is
        // actually very ugly...

        let mag = base_to_apex.magnitude();
        let dr = self.base_radius - self.apex_radius;

        let a = ray_uvw.dx * ray_uvw.dx +
            ray_uvw.dy * ray_uvw.dy -
            dr * dr * ray_uvw.dz * ray_uvw.dz / (mag * mag);

        let b = 2.0 * src_uvw.dx * ray_uvw.dx +
            2.0 * src_uvw.dy * ray_uvw.dy -
            2.0 * dr * dr * src_uvw.dz * ray_uvw.dz / (mag * mag) +
            2.0 * self.base_radius * dr * ray_uvw.dz / mag;

        let c = src_uvw.dx * src_uvw.dx +
            src_uvw.dy * src_uvw.dy -
            self.base_radius * self.base_radius -
            dr * dr * src_uvw.dz * src_uvw.dz / (mag * mag) +
            2.0 * self.base_radius * dr * src_uvw.dz / mag;
    
        let b2m4ac = b * b - 4.0 * a * c;
        if b2m4ac >= 0.0 {
            let sq = b2m4ac.sqrt();
            let r1 = (-b - sq) / (2.0 * a);
            let r2 = (-b + sq) / (2.0 * a);
            
            // The intersection point is located at rn * ray_uvw + src_uvw
            // in the new space, but it's only guaranteed to be on the
            // infinitely extended cone.  We need to check if it's beyond
            // the ends as defined by the object.  Fortunately all we need
            // to do is check the w-coordinate in the new space.

            let w1 = r1 * ray_uvw.dz + src_uvw.dz;
            let w2 = r2 * ray_uvw.dz + src_uvw.dz;

            let result =
                if r1 < r2 && r1 >= near && w1 >= 0.0 && w1 <= mag {
                    r1
                } else if r2 >= near && w2 >= 0.0 && w2 <= mag {
                    r2
                } else {
                    -1.0
                };

            if result >= near {
                let normal_uvw = Vector {
                    dx: (result * ray_uvw.dx + src_uvw.dx) * mag,
                    dy: (result * ray_uvw.dy + src_uvw.dy) * mag,
                    dz: dr
                };

                let normal = Vector {
                    dx: normal_uvw.dx * u.dx + 
                        normal_uvw.dy * v.dx + normal_uvw.dz * w.dx,
                    dy: normal_uvw.dx * u.dy +
                        normal_uvw.dy * v.dy + normal_uvw.dz * w.dy,
                    dz: normal_uvw.dx * u.dz +
                        normal_uvw.dy * v.dz + normal_uvw.dz * w.dz
                };

                return Some(IntersectResult {
                    normal: normal.normalized(),
                    dist: result
                });
            }
        }

        None
    }
}