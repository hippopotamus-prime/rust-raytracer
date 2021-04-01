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
        // Ideally the box surrounds the central line of the cone and is
        // extended out by the x/y/z components of the base and apex to form
        // a tight bound. This can be calculated using projections:
        // - Project a unit vector from each axis onto the plane of the base,
        //   e.g. if N is the normalized center line from base to apex, then
        //   for the x-axis:
        //      Px = {1, 0, 0} - N * ({1, 0, 0} dot N)
        //         = {1, 0, 0} - N * N.dx
        // - Normalize the projection and scale to the length of the base or
        //   apex.
        // - Take the axial component of each scaled projection.
        // - For the special case where an axis projects to a point on the
        //   base, the box is flush against the base (no extension).

        let unit_x = Vector {dx: 1.0, dy: 0.0, dz: 0.0};
        let unit_y = Vector {dx: 0.0, dy: 1.0, dz: 0.0};
        let unit_z = Vector {dx: 0.0, dy: 0.0, dz: 1.0};
        let n = (&self.apex - &self.base).normalized();

        let px = unit_x - (&n * n.dx);
        let mx = px.magnitude();
        let scale_x = if mx > 0.0 {
            px.dx / mx 
        } else {
            0.0
        };

        let py = unit_y - (&n * n.dy);
        let my = py.magnitude();
        let scale_y = if my > 0.0 {
            py.dy / my
        } else {
            0.0
        };

        let pz = unit_z - (&n * n.dz);
        let mz = pz.magnitude();
        let scale_z = if mz > 0.0 {
            pz.dz / mz
        } else {
            0.0
        };

        let base_x_extra = self.base_radius * scale_x;
        let apex_x_extra = self.apex_radius * scale_x;

        let base_y_extra = self.base_radius * scale_y;
        let apex_y_extra = self.apex_radius * scale_y;

        let base_z_extra = self.base_radius * scale_z;
        let apex_z_extra = self.apex_radius * scale_z;

        let min_corner_x = (self.base.x - base_x_extra).min(
            self.apex.x - apex_x_extra);
        let max_corner_x = (self.base.x + base_x_extra).max(
            self.apex.x + apex_x_extra);

        let min_corner_y = (self.base.y - base_y_extra).min(
            self.apex.y - apex_y_extra);
        let max_corner_y = (self.base.y + base_y_extra).max(
            self.apex.y + apex_y_extra);

        let min_corner_z = (self.base.z - base_z_extra).min(
            self.apex.z - apex_z_extra);
        let max_corner_z = (self.base.z + base_z_extra).max(
            self.apex.z + apex_z_extra);

        BoundingBox {
            corner: Point {
                x: min_corner_x,
                y: min_corner_y,
                z: min_corner_z
            },
            extent: Vector {
                dx: max_corner_x - min_corner_x,
                dy: max_corner_y - min_corner_y,
                dz: max_corner_z - min_corner_z
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