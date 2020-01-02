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
            let back_face = normal.dot(ray) > 0.0;
            let mut total_color = Color::black();

            // Surfaces are one-sided and invisible if viewed from the back.
            // However, refracted rays will still hit back faces, so we can't
            // ignore them completely.
            if !back_face {
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
    
                        total_color += reflected_color *
                            surface.get_reflectance();
                    }
                }
            }

            if depth < MAX_DEPTH {
                // TO DO:  This doesn't account for the thickness of the
                // intersected object, but a physically accurate rendering
                // should.
                let transmittance =
                    if back_face {
                        // Special case - the back faces of fully opaque
                        // surfaces have zero transmittance, but other surfaces
                        // transmit fully. This allows rays to exit translucent
                        // solids cleanly, but makes backwards opaque surfaces
                        // show up as black.
                        if surface.get_transmittance() > MIN_CONTRIBUTION {
                            1.0
                        } else {
                            0.0
                        }
                    } else {
                        surface.get_transmittance()
                    };
                let refraction_contribution = contribution * transmittance;
                if refraction_contribution > MIN_CONTRIBUTION {
                    let refracted_ray =
                        if back_face {
                            ray.refracted(&-normal,
                                1.0 / surface.get_refraction_index())
                        } else {
                            ray.refracted(&normal,
                                surface.get_refraction_index())
                        };

                    // TO DO:  Ignoring the intersected primitive doesn't work
                    // here since a refracted ray can hit the same primitive
                    // twice. The 0.0001 near distance avoids "refraction
                    // acne", but the ideal solution is to pass Some(primitive)
                    // and somehow only ignore the near-side intersection.
                    let refracted_color = self.sub_trace(
                        &surface_position,
                        &refracted_ray,
                        0.0001,
                        None,
                        refraction_contribution,
                        depth + 1);

                        total_color += refracted_color * transmittance;
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
