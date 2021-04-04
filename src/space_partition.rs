use std::ops::Deref;
use crate::vector_math::{Axis, Point, Vector};
use crate::shape::Shape;
use crate::shape::BoundingBox;
use crate::render::Primitive;


struct InteriorNode<'a> {
    over: Box<SpacePartition<'a>>,
    under: Box<SpacePartition<'a>>,
    axis: Axis,
    plane: f32
}

enum ChildNode<'a> {
    Leaf(Vec<&'a Primitive>),
    Interior(InteriorNode<'a>)
}

pub struct SpacePartition<'a> {
    bounding_box: BoundingBox,
    child: ChildNode<'a>
}

struct SplitAppraisal {
    under_box: Option<BoundingBox>,
    over_box: Option<BoundingBox>,
    cost: f32
}

struct SplitDecision {
    under_box: BoundingBox,
    over_box: BoundingBox,
    plane: f32
}

#[derive(Clone)]
struct BoxedPrimitive<'a> (&'a Primitive, BoundingBox);

fn find_splitting_plane(primitives: &[BoxedPrimitive],
        axis: Axis,
        no_split_cost: f32) -> Option<SplitDecision> {

    if primitives.len() < 4 {
        return None;
    }

    println!("Partitioning {} primitives on {:?}", primitives.len(), axis);

    let mut min_cost = no_split_cost;
    let mut best_plane = 0.0;
    let mut best_over_box: Option<BoundingBox> = None;
    let mut best_under_box: Option<BoundingBox> = None;

    for BoxedPrimitive(_, bounding_box) in primitives {
        let plane = bounding_box.min_corner().component(axis);
        let appraisal = appraise_split(primitives, axis, plane);

        if appraisal.cost < min_cost {
            best_under_box = appraisal.under_box;
            best_over_box = appraisal.over_box;
            best_plane = plane;
            min_cost = appraisal.cost;
        }

        let plane = bounding_box.max_corner().component(axis);
        let appraisal = appraise_split(primitives, axis, plane);

        if appraisal.cost < min_cost {
            best_under_box = appraisal.under_box;
            best_over_box = appraisal.over_box;
            best_plane = plane;
            min_cost = appraisal.cost;
        }
    }

    // Don't do any split that would put all the primitives on one side.
    // We have to have both an under and over box.
    let best_over_box = match best_over_box {
        None => return None,
        Some(bounding_box) => bounding_box
    };
    let best_under_box = match best_under_box {
        None => return None,
        Some(bounding_box) => bounding_box
    };

    if min_cost < no_split_cost {
        Some(SplitDecision {
            under_box: best_under_box,
            over_box: best_over_box,
            plane: best_plane
        })
    } else {
        None
    }
}

// Determine the cost of splitting a set of primitives on a given plane
fn appraise_split(
    primitives: &[BoxedPrimitive],
    axis: Axis,
    plane: f32) -> SplitAppraisal {

    let mut under_count = 0;
    let mut over_count = 0;
    let mut under_box: Option<BoundingBox> = None;
    let mut over_box: Option<BoundingBox> = None;

    for BoxedPrimitive(_, bounding_box) in primitives {
        if bounding_box.min_corner().component(axis) < plane {
            under_count += 1;
            let new_under_box = match under_box {
                Some(under_box) => under_box.expand_to_fit(bounding_box),
                None => bounding_box.clone()
            };
            under_box = Some(new_under_box);
        }

        if bounding_box.max_corner().component(axis) >= plane {
            over_count += 1;
            let new_over_box = match over_box {
                Some(over_box) => over_box.expand_to_fit(bounding_box),
                None => bounding_box.clone()
            };
            over_box = Some(new_over_box);
        }
    }

    let cost = match under_box.as_ref() {
        Some(under_box) => match over_box.as_ref() {
            Some(over_box) => {
                appraise(under_count, &under_box) +
                appraise(over_count, &over_box)
            },
            None => appraise(under_count, &under_box)
        },
        None => match over_box.as_ref() {
            Some(over_box) => appraise(over_count, &over_box),
            None => 0.0
        }
    };

    SplitAppraisal {
        over_box: over_box,
        under_box: under_box,
        cost: cost
    }
}

fn split<'a>(
    boxed_primitives: &[BoxedPrimitive<'a>],
    axis: Axis,
    plane: f32) ->
        (Vec<BoxedPrimitive<'a>>, Vec<BoxedPrimitive<'a>>) {

    let mut over: Vec<BoxedPrimitive<'a>> = vec![];
    let mut under: Vec<BoxedPrimitive<'a>> = vec![];

    for boxed_primitive in boxed_primitives {
        let BoxedPrimitive(_, bounding_box) = boxed_primitive;

        if bounding_box.max_corner().component(axis) > plane {
            over.push(boxed_primitive.clone());
        }
        if bounding_box.min_corner().component(axis) <= plane {
            under.push(boxed_primitive.clone());
        }
    }

    (over, under)
}

fn advance(axis: Axis) -> Axis {
    match axis {
        Axis::X => Axis::Y,
        Axis::Y => Axis::Z,
        Axis::Z => Axis::X
    }
}

// Calculate the cost of a possible partition node, assuming it contains the
// given number of primitives, all inside the given bounding box.
fn appraise(primitive_count: usize, bounding_box: &BoundingBox) -> f32 {
    // The cost of a partition should reflect the amount of computation needed
    // to trace a ray through it. Each primitive in the partition adds another
    // intersection calculation, so the cost is proportional to the number
    // of primitives. The surface area is factored in because it's roughly
    // proportional to the probability of intersecting the partition in general.
    // Rays are less likely to hit small partitions, so they're more acceptable
    // computationally.
    bounding_box.surface_area() * primitive_count as f32
}

impl<'a> SpacePartition<'a> {

    pub fn from_primitives(
            primitives: &'a[Primitive]) -> SpacePartition<'a> {

        if primitives.is_empty() {
            SpacePartition {
                bounding_box: BoundingBox::zero(),
                child: ChildNode::Leaf(vec![])
            }
        } else {
            let first_box = primitives[0].shape.bounding_box();

            let mut boxed_primitives =
                vec![BoxedPrimitive(&primitives[0], first_box.clone())];
            let mut total_box = first_box;

            for primitive in &primitives[1..] {
                let bounding_box = primitive.shape.bounding_box();
                total_box = total_box.expand_to_fit(&bounding_box);
                boxed_primitives.push(BoxedPrimitive(&primitive, bounding_box));
            }

            SpacePartition::from_boxed_primitives(
                &boxed_primitives, Axis::X, total_box)
        }
    }

    fn from_boxed_primitives(
            boxed_primitives: &[BoxedPrimitive<'a>],
            axis: Axis,
            bounding_box: BoundingBox) -> SpacePartition<'a> {

        let no_split_cost = appraise(boxed_primitives.len(), &bounding_box);
        let decision = find_splitting_plane(&boxed_primitives, axis, no_split_cost);
        match decision {
            None => {
                let primitives: Vec<_> = boxed_primitives.into_iter().map(
                    |BoxedPrimitive(primitive, _)| *primitive).collect();
                SpacePartition {
                    bounding_box: bounding_box,
                    child: ChildNode::Leaf(primitives)
                }
            },
            Some(SplitDecision {under_box, over_box, plane}) => {
                let (over, under) = split(boxed_primitives, axis, plane);
                let next_axis = advance(axis);

                let over = Box::new(SpacePartition::from_boxed_primitives(
                    &over, next_axis, over_box));
                let under = Box::new(SpacePartition::from_boxed_primitives(
                    &under, next_axis, under_box));

                SpacePartition {
                    bounding_box: bounding_box,
                    child: ChildNode::Interior(InteriorNode {
                        over: over,
                        under: under,
                        axis: axis,
                        plane: plane
                })}
            }
        }
    }

    // Given `ray` originating from `src`, find the primitive in the scene
    // that the ray intersects. If an intersection is found, the return a
    // tuple of: the surface normal at the intersection point, the distance to
    // the intersection point, and the primitive that was intersected.
    //
    // `near` is a near-clipping distance.
    //
    // `ignore` is a primitive to ignore when calculating intersections.
    pub fn intersect(&self,
        src: &Point,
        ray: &Vector,
        near: f32,
        ignore: Option<&dyn Shape>) ->
            Option<(Vector, f32, &Primitive)> {

        // Quick test - does the ray hit the bounding box for this partition?
        if !self.bounding_box.intersect(src, ray, near) {
            // No intersection possible if the ray missed the bounding box.
            return None;
        }
        
        match &self.child {
            ChildNode::Leaf(primitives) => {
                intersect_primitives(primitives, src, ray, near, ignore)
            },
            ChildNode::Interior(node) => {
                node.intersect(src, ray, near, ignore)
            }
        }
    }
}

impl<'a> InteriorNode<'a> {
    fn intersect(&self,
        src: &Point,
        ray: &Vector,
        near: f32,
        ignore: Option<&dyn Shape>) ->
            Option<(Vector, f32, &Primitive)> {

        // Intersect whichever sub-partition the ray starts in first, then
        // hopefully skip the other one.

        if src.component(self.axis) < self.plane {
            // Starting on the under side of the plane.
            let under_result = self.under.intersect(src, ray, near, ignore);

            // Need to check the other side in two cases:
            // - If the ray didn't hit anything, obviously.
            // - If the ray hit something that spans both halves and
            //   the intersection is on the other side of the splitting
            //   plane; in this case we can't say whether or not the
            //   found intersection is actually the closest one.
            let check_over = match under_result {
                None => true,
                Some((_, distance, _)) => {
                    let endpoint = src + ray * distance;
                    endpoint.component(self.axis) > self.plane
                }
            };

            if check_over {
                self.over.intersect(src, ray, near, ignore)
            } else {
                under_result
            }
        } else {
            // Starting on the over side of the plane.
            let over_result =
                self.over.intersect(src, ray, near, ignore);

            let check_under = match over_result {
                None => true,
                Some((_, distance, _)) => {
                    let endpoint = src + ray * distance;
                    endpoint.component(self.axis) < self.plane
                }
            };

            if check_under {
                self.under.intersect(src, ray, near, ignore)
            } else {
                over_result
            }
        }
    }
}

fn intersect_primitives<'a>(
    primitives: &Vec<&'a Primitive>,
    src: &Point,
    ray: &Vector,
    near: f32,
    ignore: Option<&dyn Shape>) ->
        Option<(Vector, f32, &'a Primitive)> {

    // Test all the prmitives using a linear search and return the nearest
    // intersection.
    let mut best_result: Option<(Vector, f32, &Primitive)> = None;

    for primitive in primitives {
        if let Some(ignored_shape) = ignore {
            if ignored_shape as *const _ ==
                    primitive.shape.deref() as *const _ {
                continue;
            }
        }

        if let Some(intersection) =
                primitive.shape.intersect(src, ray, near) {
            let better_result_found = match &best_result {
                Some((_, prior_nearest, _)) =>
                    intersection.dist < *prior_nearest,
                None =>
                    true
            };

            if better_result_found {
                best_result = Some((intersection.normal,
                    intersection.dist,
                    primitive));
            }
        }
    }
    best_result
}
