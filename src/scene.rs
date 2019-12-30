use std::rc::Rc;
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
        let intersection = self.intersect_surface(src, ray, near);

        if let Some((normal, distance, surface)) = intersection {
            let surface_position = src + ray * distance;
            let mut total_color = Color {r: 0.0, g: 0.0, b: 0.0};
            for light in &self.lights {
                let mut surface_to_light = &light.position - &surface_position;
                surface_to_light.normalize();
    
                let color = surface.get_visible_color(
                    &normal, ray, &surface_to_light, &light.color);
    
                total_color.r += color.r;
                total_color.g += color.g;
                total_color.b += color.b;
            }
            total_color.clamp();
            return total_color;
        }
    
        self.background.clone()
    }

    fn intersect_surface(&self, src: &Point, ray: &Vector, near: f32) ->
            Option<(Vector, f32, Rc<dyn Surface>)> {
        let mut best_result: Option<(Vector, f32, Rc<dyn Surface>)> = None;

        for (primitive, surface) in &self.primitives {
            if let Some(intersection) = primitive.intersect(src, ray, near) {
                match &best_result {
                    Some((_, prior_nearest, _)) => {
                        if intersection.dist < *prior_nearest {
                            best_result = Some((intersection.normal,
                                intersection.dist, surface.clone()));
                        }
                    },
                    None => {
                        best_result = Some((intersection.normal,
                            intersection.dist, surface.clone()));
                    }
                }
            }
        }
        best_result
    }
}
