use crate::vector_math;
use crate::vector_math::Vector;
use crate::vector_math::Point;
use crate::intersect::Intersect;
use crate::intersect::IntersectResult;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

pub struct View {
    pub from: Point,
    pub at: Point,
    pub up: Vector,
    pub angle: f32,
    pub hither: f32,
    pub width: u32,
    pub height: u32
}

impl View {
    pub fn aspect_ratio(&self) -> f32 {
        (self.width as f32) / (self.height as f32)
    }
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

impl Intersect for Scene {
    fn intersect(&self, src: &Point, ray: &Vector, near: f32) ->
            Option<IntersectResult> {
        for primitive in &self.primitives {
            if let Some(result) = primitive.intersect(src, ray, near) {
                return Some(result);
            }
        }
        None
    }
}

pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    values: Vec<Color>
}

impl RenderTarget {
    pub fn new(width: usize, height: usize) -> RenderTarget {
        let mut values = Vec::new();
        values.resize(width * height, Color {r: 0.0, g: 0.0, b: 0.0});
        RenderTarget {width, height, values}
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.values[y * self.height + x] = color;
    }

    pub fn get(&self, x: usize, y: usize) -> &Color {
        &self.values[y * self.height + x]
    }
}

pub fn render(view: &View, scene: &Scene, target: &mut RenderTarget) {
    // All the rays can be thought of as passing through a rectangular screen
    // that is <near> away from the eye, with dimensions:
    //      width:  aspect ratio * near * tan(fov/2)
    //      height: near * tan(fov/2).
    // Since we're normalizing the rays anyway, we can assume <near> is
    // a distance of 1.0 in the direction <at> - <from>.

    // Distance from the center of the screen to either edge
    let up_len = (std::f32::consts::PI * view.angle / 360.0).tan();
    let right_len = up_len * view.aspect_ratio();

    // Vector from the eye to the center of the screen.
    let forward = (&view.at - &view.from).normalized();

    // Vector from the center of the screen to the right edge.
    let right = vector_math::cross(&forward, &view.up).normalized() * right_len;

    // Vector from the center of the screen to the top edge; note view.up may
    // not be perpendicular to forward, but this is.
    let up = vector_math::cross(&right, &forward).normalized() * up_len;

    for j in 0..target.height {
        println!("Rendering line {}", j + 1);

        // Convert to screen coordinates in the range [-1.0, 1.0]
        let sy = 1.0 - ((2 * (j as isize) - 1) as f32) /
            (target.height as f32);

        for i in 0..target.width {
            let sx = 1.0 - ((2 * (i as isize) - 1) as f32) /
                (target.width as f32);

            let ray = (&forward + &up * sy + &right * sx).normalized();
            let color = trace(&view.from, &ray, &scene, view.hither);
            target.set(i, j, color);
        }
    }
}

fn trace(src: &Point, ray: &Vector, scene: &Scene, near: f32) -> Color {
    let intersect_result = scene.intersect(src, ray, near);
    if let None = intersect_result {
        return scene.background.clone();
    }

    // TO DO:  Actual math and stuff to find the right color
    Color {
        r: 1.0,
        g: 1.0,
        b: 1.0
    }
}
