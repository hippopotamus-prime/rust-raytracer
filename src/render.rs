use crate::vector_math::Vector;
use crate::vector_math::Point;
use crate::intersect::Intersect;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

pub struct View {
    pub from: Point,
    pub up: Vector,
    pub at: Vector,
    pub angle: f32,
    pub hither: f32,
    pub width: u32,
    pub height: u32
}

pub struct Scene {
    pub background: Color,
    pub primitives: Vec<Box<dyn Intersect>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            background: Color {r: 1.0, g: 1.0, b: 1.0},
            primitives: vec! {}
        }
    }
}

pub struct RenderTarget {

}

pub fn render(view: &View, scene: &Scene, target: &mut RenderTarget) {
    // TO DO: Loop over each pixel and trace rays
    // - start by drawing the background color
    // - do some math to intersect polygons
}
