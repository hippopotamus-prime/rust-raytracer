use crate::vector_math::{Point, Vector, Axis};

pub struct IntersectResult {
    pub normal: Vector,
    pub dist: f32
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub corner: Point,
    pub extent: Vector
}

pub trait Shape {
    fn intersect(&self, src: &Point, ray: &Vector, near: f32) ->
            Option<IntersectResult>;

    fn bounding_box(&self) -> BoundingBox;
}

impl BoundingBox {
    pub fn zero() -> BoundingBox {
        BoundingBox {
            corner: Point::origin(),
            extent: Vector {
                dx: 0.0,
                dy: 0.0,
                dz: 0.0
            }
        }
    }

    pub fn min_corner(&self) -> &Point {
        &self.corner
    }

    pub fn max_corner(&self) -> Point {
        &self.corner + &self.extent
    }

    // pub fn volume(&self) -> f32 {
    //     self.extent.dx * self.extent.dy * self.extent.dz
    // }

    pub fn surface_area(&self) -> f32 {
        self.extent.dx * self.extent.dy * 2.0 +
        self.extent.dy * self.extent.dz * 2.0 +
        self.extent.dx * self.extent.dz * 2.0
    }

    pub fn face_area(&self, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.extent.dy * self.extent.dz,
            Axis::Y => self.extent.dx * self.extent.dz,
            Axis::Z => self.extent.dx * self.extent.dy
        }
    }

    pub fn expand_to_fit(&self, other: &BoundingBox) -> BoundingBox {
        let min_x = self.corner.x.min(other.corner.x);
        let min_y = self.corner.y.min(other.corner.y);
        let min_z = self.corner.z.min(other.corner.z);

        let self_far_corner = &self.corner + &self.extent;
        let other_far_corner = &other.corner + &other.extent;
        let max_x = self_far_corner.x.max(other_far_corner.x);
        let max_y = self_far_corner.y.max(other_far_corner.y);
        let max_z = self_far_corner.z.max(other_far_corner.z);

        BoundingBox {
            corner: Point {
                x: min_x,
                y: min_y,
                z: min_z
            },
            extent: Vector {
                dx: max_x - min_x,
                dy: max_y - min_y,
                dz: max_z - min_z
            }
        }
    }

    pub fn intersect(&self, src: &Point, ray: &Vector, near_cull: f32) -> bool {

        // TO DO - speed this up using the stuff from the Pluecker paper.

        // Basic idea - consider the box as the intersection of three "slabs"
        // in space.  The ray intersects each slab twice, at a near plane and a
        // far plane.  If the first of the far plane intersections comes before
        // the last near plane intersection, the ray misses the box.

        let mut largest_near = std::f32::MIN;
        let mut smallest_far = std::f32::MAX;

        let src_to_min_corner = &self.corner - src;
        let src_to_max_corner = &self.corner + &self.extent - src;

        if ray.dx != 0.0 {
            // Where does the ray hit the x-planes?
            let to_min_plane = src_to_min_corner.dx / ray.dx;
            let to_max_plane = src_to_max_corner.dx / ray.dx;

            // Depending on the ray direction, pick the which one will be hit
            // first and last (i.e. near and far)
            if to_min_plane < to_max_plane {
                largest_near = to_min_plane;
                smallest_far = to_max_plane;
            } else {
                largest_near = to_max_plane;
                smallest_far = to_max_plane;
            }
        } else if src.x < self.corner.x ||
                        src.x > self.corner.x + self.extent.dx {
            // Oh, the ray doesn't actually intersect the x planes...  then if
            // the ray doesn't start out between them, it can't possibly hit
            // the box.
            return false;
        }

        if ray.dy != 0.0 {
            let to_min_plane = src_to_min_corner.dy / ray.dy;
            let to_max_plane = src_to_max_corner.dy / ray.dy;

            let (near, far) = if to_min_plane < to_max_plane {
                (to_min_plane, to_max_plane)
            } else {
                (to_max_plane, to_min_plane)
            };

            // See if these are the final near intersection or the first far
            // intersection.
            if near > largest_near {
                largest_near = near;
            }
            if far < smallest_far {
                smallest_far = far;
            }
        } else if src.y < self.corner.y ||
                        src.y > self.corner.y + self.extent.dy {
            return false;
        }

        if ray.dz != 0.0 {
            let to_min_plane = src_to_min_corner.dz / ray.dz;
            let to_max_plane = src_to_max_corner.dz / ray.dz;

            let (near, far) = if to_min_plane < to_max_plane {
                (to_min_plane, to_max_plane)
            } else {
                (to_max_plane, to_min_plane)
            };

            if near > largest_near {
                largest_near  = near;
            }
            if far < smallest_far {
                smallest_far = far;
            }
        } else if src.z < self.corner.z ||
                        src.z > self.corner.z + self.extent.dz {
            return false;
        }

        // So, not only does the first far plane intersection have to be
        // farther away than the last near plane intersection, but it also has
        // to be in front of the ray starting point...
        return smallest_far > largest_near && smallest_far >= near_cull;
    }
}
