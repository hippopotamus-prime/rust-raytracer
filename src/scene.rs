use std::rc::Rc;
use std::ops::Deref;
use crate::vector_math::Point;
use crate::vector_math::Vector;
use crate::render::Color;
use crate::render::Surface;
use crate::intersect::Intersect;

#[derive(Debug, Clone)]
pub struct Light {
    pub position: Point,
    pub color: Color
}

pub struct Scene {
    pub background: Color,
    lights: Vec<Light>,
    primitives: Vec<(Box<dyn Intersect>, Rc<dyn Surface>)>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            background: Color {r: 1.0, g: 1.0, b: 1.0},
            lights: vec! {},
            primitives: vec! {}
        }
    }

    pub fn add_primitive(&mut self,
            primitive: Box<dyn Intersect>,
            surface: Rc<dyn Surface>) {
        self.primitives.push((primitive, surface));
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn trace(&self, src: &Point, ray: &Vector, near: f32) -> Color {
        let intersection = self.intersect_surface(src, ray, near, None);

        if let Some((normal, distance, surface, primitive)) = intersection {
            let surface_position = src + ray * distance;
            let mut total_color = Color {r: 0.0, g: 0.0, b: 0.0};

            for light in &self.lights {
                let surface_to_light = &light.position - &surface_position;
                let light_distance = surface_to_light.magnitude();
                let light_direction = surface_to_light / light_distance;

                let light_blocked = match self.intersect_surface(
                        &surface_position,
                        &light_direction,
                        0.0,
                        Some(primitive)) {
                    Some((_, blocker_distance, _, _)) => {
                        blocker_distance <= light_distance
                    },
                    None => false
                };

                if !light_blocked {
                    let color = surface.get_visible_color(
                        &normal, ray, &light_direction, &light.color);

                    total_color.r += color.r;
                    total_color.g += color.g;
                    total_color.b += color.b;
                }
            }
            total_color.clamp();
            return total_color;
        }
    
        self.background.clone()
    }

    fn intersect_surface(&self,
        src: &Point,
        ray: &Vector,
        near: f32,
        ignore: Option<&dyn Intersect>) ->
                    Option<(Vector, f32, Rc<dyn Surface>, &dyn Intersect)> {
        let mut best_result:
            Option<(Vector, f32, Rc<dyn Surface>, &dyn Intersect)> = None;

        for (primitive, surface) in &self.primitives {
            if let Some(ignored_primitive) = ignore {
                if ignored_primitive as *const _ ==
                        primitive.deref() as *const _ {
                    continue;
                }
            }

            if let Some(intersection) = primitive.intersect(src, ray, near) {
                let better_result_found = match &best_result {
                    Some((_, prior_nearest, _, _)) =>
                        intersection.dist < *prior_nearest,
                    None =>
                        true
                };

                if better_result_found {
                    best_result = Some((intersection.normal,
                        intersection.dist, surface.clone(), primitive.deref()));
                }
            }
        }
        best_result
    }
}
