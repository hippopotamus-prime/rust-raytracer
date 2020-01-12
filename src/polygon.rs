use crate::vector_math;
use crate::vector_math::{Point, Vector, PointNormal};
use crate::shape::{Shape, IntersectResult, BoundingBox};


pub struct Polygon {
    pub vertices: Vec<PointNormal>
}

impl Shape for Polygon {
    fn  bounding_box(&self) -> BoundingBox {
        let mut min_x = self.vertices[0].point.x;
        let mut max_x = min_x;
        let mut min_y = self.vertices[0].point.y;
        let mut max_y = min_y;
        let mut min_z = self.vertices[0].point.z;
        let mut max_z = min_z;

        for vertex in &self.vertices[1..] {
            if vertex.point.x < min_x {
                min_x = vertex.point.x;
            }
            if vertex.point.x > max_x {
                max_x = vertex.point.x;
            }
            if vertex.point.y < min_y {
                min_y = vertex.point.y;
            }
            if vertex.point.y > max_y {
                max_y = vertex.point.y;
            }
            if vertex.point.z < min_z {
                min_z = vertex.point.z;
            }
            if vertex.point.z > max_z {
                max_z = vertex.point.z;
            }
        }

        // TO DO: Does this work just as well?
        // let min_x = self.vertices.iter().map(|pn| pn.point.x).min();
        // ...

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

    fn intersect(&self, src: &Point, ray: &Vector, near: f32) ->
            Option<IntersectResult> {

        let edge1 = &self.vertices[1].point - &self.vertices[0].point;
        let edge2 = &self.vertices[2].point - &self.vertices[0].point;
        let geometric_normal = vector_math::cross(&edge1, &edge2);

        let den = vector_math::dot(&ray, &geometric_normal);
        if den.abs() < 0.000001 {
            // This means the ray is (very nearly) parallel to the plane of
            // the polygon - no intersection possible.
            return None;
        }

        let to_v1 = &self.vertices[0].point - src;
        let num = vector_math::dot(&to_v1, &geometric_normal);
        let src_to_plane_dist = num / den;

        if src_to_plane_dist < near {
            // The distance to the polygon's plane is less than the near
            // view plane - ingore the intersection.
            return None;
        }

        // The general approach here is to draw another ray away from the point
        // where the ray intersects the polygon's plane, then count the number
        // of edges intersected - an odd number means intersection point was
        // inside the polygon.  Unlike some other methods, this works with
        // non-convex polygons.

        // To simplify the math we'll project the polygon onto an axis-aligned
        // plane, determined by the smaller two components of the geometric
        // normal.

        // TO DO:  A better approach for this would be to change the basis
        // vectors for the polygon vertices, such that all the points are in
        // a 2D plane with index 0 at (0, 0) and index 1 at (x, 0).  Then you
        // wouldn't need the 3 projection cases and the whole thing would
        // probably be more numerically stable.  There is probably also a
        // better way to do the normal interpolation - the current method
        // ignores most of the edge data.

        let in_plane = src + ray * src_to_plane_dist;
        let mut edge_intersection_count: u32 = 0;
        let mut nearest_forward_dist: Option<f32> = None;
        let mut nearest_forward_scale = 0.0;
        let mut nearest_forward_index = 0;
        let mut nearest_reverse_dist: Option<f32> = None;
        let mut nearest_reverse_scale = 0.0;
        let mut nearest_reverse_index = 0;

        if geometric_normal.dz.abs() > geometric_normal.dx.abs()
        && geometric_normal.dz.abs() > geometric_normal.dy.abs() {
            // Largest normal component is z, so polygon's major plane is x-y.
            // We'll trace along x.

            for i in 0..self.vertices.len() {
                let point = &self.vertices[i].point;
                let next_point =
                    &self.vertices[(i + 1) % self.vertices.len()].point;
                let edge = point - next_point;

                if edge.dy.abs() < 0.000001 {
                    // The edge is (very nearly) parallel to our trace - no
                    // intersection possible.
                    continue;
                }

                let scale = (in_plane.y - next_point.y) / edge.dy;
                if scale < 0.0 || scale > 1.0 {
                    // Scale is the relative position between the two edge
                    // endpoints - here we're outside both, so no intersection.
                    continue;
                }

                let to_edge_dist = scale * edge.dx + next_point.x - in_plane.x;
                if to_edge_dist >= 0.0 {
                    edge_intersection_count += 1;

                    if let Some(dist) = nearest_forward_dist {
                        if to_edge_dist < dist {
                            nearest_forward_dist = None
                        }
                    }
                    if let None = nearest_forward_dist {
                        nearest_forward_dist = Some(to_edge_dist);
                        nearest_forward_index = i;
                        nearest_forward_scale = scale;
                    }
                } else {
                    // The edge intersection is backwards along the trace.
                    // This won't count for an intersection, but we need to
                    // track which edge is closest to calculate a final
                    // result.

                    if let Some(dist) = nearest_reverse_dist {
                        if to_edge_dist > dist {
                            nearest_reverse_dist = None
                        }
                    }
                    if let None = nearest_reverse_dist {
                        nearest_reverse_dist = Some(to_edge_dist);
                        nearest_reverse_index = i;
                        nearest_reverse_scale = scale;
                    }
                }
            }
        } else if geometric_normal.dy.abs() > geometric_normal.dx.abs() {
            // Largest normal component is y, so polygon's major plane is x-z.
            // Trace along x again...

            for i in 0..self.vertices.len() {
                let point = &self.vertices[i].point;
                let next_point =
                    &self.vertices[(i + 1) % self.vertices.len()].point;
                let edge = point - next_point;

                if edge.dz.abs() < 0.000001 {
                    continue;
                }

                let scale = (in_plane.z - next_point.z) / edge.dz;
                if scale < 0.0 || scale > 1.0 {
                    continue;
                }

                let to_edge_dist = scale * edge.dx + next_point.x - in_plane.x;
                if to_edge_dist >= 0.0 {
                    edge_intersection_count += 1;

                    if let Some(dist) = nearest_forward_dist {
                        if to_edge_dist < dist {
                            nearest_forward_dist = None
                        }
                    }
                    if let None = nearest_forward_dist {
                        nearest_forward_dist = Some(to_edge_dist);
                        nearest_forward_index = i;
                        nearest_forward_scale = scale;
                    }
                } else {
                    if let Some(dist) = nearest_reverse_dist {
                        if to_edge_dist > dist {
                            nearest_reverse_dist = None
                        }
                    }
                    if let None = nearest_reverse_dist {
                        nearest_reverse_dist = Some(to_edge_dist);
                        nearest_reverse_index = i;
                        nearest_reverse_scale = scale;
                    }
                }
            }
        } else {
            // Largest normal component is x, so polygon's major plane is y-z.
            // Trace along y.

            for i in 0..self.vertices.len() {
                let point = &self.vertices[i].point;
                let next_point =
                    &self.vertices[(i + 1) % self.vertices.len()].point;
                let edge = point - next_point;

                if edge.dz.abs() < 0.000001 {
                    continue;
                }

                let scale = (in_plane.z - next_point.z) / edge.dz;
                if scale < 0.0 || scale > 1.0 {
                    continue;
                }

                let to_edge_dist = scale * edge.dy + next_point.y - in_plane.y;
                if to_edge_dist >= 0.0 {
                    edge_intersection_count += 1;

                    if let Some(dist) = nearest_forward_dist {
                        if to_edge_dist < dist {
                            nearest_forward_dist = None
                        }
                    }
                    if let None = nearest_forward_dist {
                        nearest_forward_dist = Some(to_edge_dist);
                        nearest_forward_index = i;
                        nearest_forward_scale = scale;
                    }
                } else {
                    if let Some(dist) = nearest_reverse_dist {
                        if to_edge_dist > dist {
                            nearest_reverse_dist = None
                        }
                    }
                    if let None = nearest_reverse_dist {
                        nearest_reverse_dist = Some(to_edge_dist);
                        nearest_reverse_index = i;
                        nearest_reverse_scale = scale;
                    }
                }
            }
        }

        if edge_intersection_count & 1 == 0 {
            // The trace hit an even number of polygon edges, meaning the
            // starting point must have been outside - so no intersection.
            return None
        }

        // Bilinearly interpolate between the nearest forward and reverse
        // edges.  Both should always be found for well-defined polygons.
        match (nearest_forward_dist, nearest_reverse_dist) {
            (Some(forward_dist), Some(reverse_dist)) => {
                let fna = &self.vertices[nearest_forward_index].normal;
                let fnb = &self.vertices[(nearest_forward_index + 1) %
                        self.vertices.len()].normal;
                let forward_normal = vector_math::interpolate(
                    fna, fnb, nearest_forward_scale).normalized();

                let rna = &self.vertices[nearest_reverse_index].normal;
                let rnb = &self.vertices[(nearest_reverse_index + 1) %
                        self.vertices.len()].normal;
                let reverse_normal = vector_math::interpolate(
                    rna, rnb, nearest_reverse_scale).normalized();

                // Remember reverse_dist is negative, forward_dist is positive.
                let scale = reverse_dist / (reverse_dist - forward_dist);
                let normal = vector_math::interpolate(
                    &forward_normal, &reverse_normal, scale).normalized();

                Some(IntersectResult {
                    normal: normal,
                    dist: src_to_plane_dist
                })
            }
            (_, _) => None
        }
    }
}
