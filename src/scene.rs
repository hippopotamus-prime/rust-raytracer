use std::rc::Rc;
use std::ops::Deref;
use crate::vector_math::Point;
use crate::vector_math::Vector;
use crate::color::Color;
use crate::render::Surface;
use crate::intersect::Intersect;

const MAX_DEPTH: u32 = 5;
const MIN_CONTRIBUTION: f32 = 0.003;

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
        self.sub_trace(src, ray, near, None, 1.0, 0)
    }

    fn sub_trace(&self,
            src: &Point,
            ray: &Vector,
            near: f32,
            ignore: Option<&dyn Intersect>,
            contribution: f32,
            depth: u32) -> Color {
        let intersection = self.intersect_surface(src, ray, near, ignore);

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
                    let direct_color = surface.get_visible_color(
                        &normal, ray, &light_direction, &light.color);

                    total_color += direct_color;
                }
            }

            if depth < MAX_DEPTH {
                let reflection_contribution =
                    contribution * surface.get_reflectance();
                if reflection_contribution > MIN_CONTRIBUTION {
                    let reflected_ray = ray.reflected(&normal);
                    let reflected_color = self.sub_trace(
                        &surface_position,
                        &reflected_ray,
                        0.0,
                        Some(primitive),
                        reflection_contribution,
                        depth + 1);

                    total_color += reflected_color * surface.get_reflectance();
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
