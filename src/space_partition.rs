use crate::vector_math::Axis;
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

    // Special cost calculation - the probability of a ray intersecting one of
    // a pair of partition nodes is roughly proportional to the pair's total
    // visible surface area, i.e. excluding the two faces that are touching
    // each other.
    let cost = match under_box.as_ref() {
        Some(under_box) => match over_box.as_ref() {
            Some(over_box) => {
                let visible_surface_area =
                    over_box.surface_area() +
                    under_box.surface_area() -
                    over_box.face_area(axis) -
                    under_box.face_area(axis);

                visible_surface_area * primitives.len() as f32
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
    // TO DO: Write code to intersect a partition

    // TO DO: Rendering the scene should involve building a partition and
    // then tracing rays through it, instead of linearly walking through the
    // scene's primitives

    pub fn from_primitives(
            primitives: &'a[Primitive]) -> Option<SpacePartition<'a>> {

        if primitives.is_empty() {
            None
        }
        else {
            let first_box = primitives[0].shape.bounding_box();

            let mut boxed_primitives =
                vec![BoxedPrimitive(&primitives[0], first_box.clone())];
            let mut total_box = first_box;

            for primitive in &primitives[1..] {
                let bounding_box = primitive.shape.bounding_box();
                total_box = total_box.expand_to_fit(&bounding_box);
                boxed_primitives.push(BoxedPrimitive(&primitive, bounding_box));
            }

            Some(SpacePartition::from_boxed_primitives(
                &boxed_primitives, Axis::X, total_box))
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
}
