use std::rc::Rc;
use crate::vector_math::{Point, Vector};
use crate::color::Color;
use crate::render::{Surface, Primitive};
use crate::shape::Shape;
use crate::space_partition::SpacePartition;

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
    primitives: Vec<Primitive>
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
            shape: Box<dyn Shape>,
            surface: Rc<dyn Surface>) {
        self.primitives.push(
            Primitive {
                shape: shape,
                surface: surface
            });
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn build_space_partition(&self) -> SpacePartition {
        SpacePartition::from_primitives(&self.primitives)
    }

    // Top-level tracing funtion. Given `ray` originating from the viewer at
    // `src`, return the color of the point that it intersects in the scene.
    //
    // `space_partition` is an acceleration structure.
    //
    // `near` is the near-clipping distance; intersections closer to `src` will
    // be ignored, meaning those parts of the scene will be invisible.
    pub fn trace(&self,
            space_partition: &SpacePartition,
            src: &Point,
            ray: &Vector,
            near: f32) -> Color {
        self.sub_trace(space_partition, src, ray, near, None, 1.0, 0)
    }

    // More detailed tracing function. Given `ray` originating from some point
    // at `src` (not necessarily the viewer), return the ray's contribution to
    // the point's visible color, based on the object the the ray intersects in
    // the scene and any subsequent reflections/refractions.
    //
    // `near` is a near-clipping distance.
    //
    // `ignore` is an optional primitive in the scene to ignore when calculating
    // intersections, typically the primitive on which `src` resides if the ray
    // is a reflection/refraction.
    //
    // `contribution` is the fraction of the point's visible color that the ray
    // will contribute. The value diminishes with each reflection/refraction
    // and tracing will stop below a minimum threshold.
    //
    // `depth` is the recursion depth in terms of reflection/refraction rays.
    // Tracing will stop at a maximum threshold.
    fn sub_trace(&self,
            space_partition: &SpacePartition,
            src: &Point,
            ray: &Vector,
            near: f32,
            ignore: Option<&dyn Shape>,
            contribution: f32,
            depth: u32) -> Color {
        let intersection = space_partition.intersect(src, ray, near, ignore);

        if let Some((normal, distance, primitive)) = intersection {
            let shape = primitive.shape.as_ref();
            let surface = primitive.surface.as_ref();
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
        
                    let light_blocked = match space_partition.intersect(
                            &surface_position,
                            &light_direction,
                            0.0,
                            Some(shape)) {
                        Some((_, blocker_distance, _)) => {
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
                            space_partition,
                            &surface_position,
                            &reflected_ray,
                            0.0,
                            Some(shape),
                            reflection_contribution,
                            depth + 1);
    
                        total_color += reflected_color *
                            primitive.surface.get_reflectance();
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
                        space_partition,
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
}
