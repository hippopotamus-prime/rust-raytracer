use crate::vector_math::{Point, Vector, Axis};
use crate::shape::BoundingBox;
use crate::render::Primitive;


struct InteriorNode<'a> {
    over: Box<SpacePartition<'a>>,
    under: Box<SpacePartition<'a>>,
    axis: Axis,
    plane: f32
}

pub enum SpacePartition<'a> {
    Leaf(Vec<&'a Primitive>),
    Interior(InteriorNode<'a>)
}

#[derive(Clone)]
struct BoxedPrimitive<'a> (&'a Primitive, BoundingBox);

fn find_splitting_plane(primitives: &[BoxedPrimitive],
            axis: Axis) -> Option<f32> {

    if primitives.len() < 4 {
        return None
    }

    let no_split_cost = appraise_full_set(primitives);
    let mut min_cost = no_split_cost;
    let mut best_plane = 0.0;
    let mut best_over_box = BoundingBox::zero();
    let mut best_under_box = BoundingBox::zero();

    for BoxedPrimitive(_, bounding_box) in primitives {
        let plane = bounding_box.min_corner().component(axis);
        let (plane_cost, plane_under_box, plane_over_box) =
            appraise_splitting_plane(primitives, axis, plane);

        if plane_cost < min_cost {
            best_under_box = plane_under_box;
            best_over_box = plane_over_box;
            best_plane = plane;
            min_cost = plane_cost;
        }

        let plane = bounding_box.max_corner().component(axis);
        let (plane_cost, plane_under_box, plane_over_box) =
            appraise_splitting_plane(primitives, axis, plane);

        if plane_cost < min_cost {
            best_under_box = plane_under_box;
            best_over_box = plane_over_box;
            best_plane = plane;
            min_cost = plane_cost;
        }
    }

    if min_cost < no_split_cost {
        Some(best_plane)
    } else {
        None
    }
}

fn appraise_full_set(boxed_primitives: &[BoxedPrimitive]) -> f32 {
    if boxed_primitives.is_empty() {
        0.0
    } else {
        let mut total_bounding_box = boxed_primitives[0].1.clone();
        for boxed_primitive in &boxed_primitives[1..] {
            let BoxedPrimitive(_, bounding_box) = boxed_primitive;
            total_bounding_box = total_bounding_box.expand_to_fit(bounding_box);
        }
        total_bounding_box.surface_area() * boxed_primitives.len() as f32
    }
}

// Determine the cost of splitting a set of primitives on a given plane
fn appraise_splitting_plane(
        primitives: &[BoxedPrimitive],
        axis: Axis,
        plane: f32) ->
            (f32, BoundingBox, BoundingBox) {
    let mut under_count = 0;
    let mut over_count = 0;

    let point_on_plane = match axis {
        Axis::X => Point {x: plane, y: 0.0, z: 0.0},
        Axis::Y => Point {x: 0.0, y: plane, z: 0.0},
        Axis::Z => Point {x: 0.0, y: 0.0, z: plane}
    };

    let mut under_box = BoundingBox {
        corner: point_on_plane.clone(),
        extent: Vector {dx: 0.0, dy: 0.0, dz: 0.0}
    };

    let mut over_box = BoundingBox {
        corner: point_on_plane,
        extent: Vector {dx: 0.0, dy: 0.0, dz: 0.0}
    };

    for BoxedPrimitive(_, bounding_box) in primitives {
        if bounding_box.min_corner().component(axis) < plane {
            under_count += 1;
            under_box = under_box.expand_to_fit(bounding_box);
        }

        if bounding_box.max_corner().component(axis) >= plane {
            over_count += 1;
            over_box = over_box.expand_to_fit(bounding_box);
        }
    }

    let cost = under_box.surface_area() * under_count as f32 +
        over_box.surface_area() * over_count as f32;

    (cost, under_box, over_box)
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

impl<'a> SpacePartition<'a> {
    // TO DO: Write code to intersect a partition

    // TO DO: Rendering the scene should involve building a partition and
    // then tracing rays through it, instead of linearly walking through the
    // scene's primitives

    pub fn from_primitives(
            primitives: &'a[Primitive]) -> SpacePartition<'a> {

        let boxed_primitives: Vec<_> = primitives.into_iter().map(
            |primitive| {
                BoxedPrimitive(primitive, primitive.shape.bounding_box())
            }).collect();

        SpacePartition::from_boxed_primitives(&boxed_primitives, Axis::X)
    }

    fn from_boxed_primitives(
        boxed_primitives: &[BoxedPrimitive<'a>],
        axis: Axis) ->
            SpacePartition<'a> {

        match find_splitting_plane(&boxed_primitives, Axis::X) {
            Some(plane) => {
                let (over, under) = split(boxed_primitives, axis, plane);
                let next_axis = advance(axis);

                let over = Box::new(
                    SpacePartition::from_boxed_primitives(&over, next_axis));
                let under = Box::new(
                    SpacePartition::from_boxed_primitives(&under, next_axis));

                SpacePartition::Interior(InteriorNode {
                    over: over,
                    under: under,
                    axis: axis,
                    plane: plane
                })
            },
            None => {
                // TO DO: Should we permanently store the bounding boxes?
                
                let primitives: Vec<_> = boxed_primitives.into_iter().map(
                    |BoxedPrimitive(primitive, _)| *primitive).collect();

                SpacePartition::Leaf(primitives)
            }
        }
    }
}
