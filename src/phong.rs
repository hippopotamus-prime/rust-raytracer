use crate::render::Color;
use crate::render::Surface;
use crate::vector_math;
use crate::vector_math::Vector;

pub struct Phong {
    pub color: Color,
    pub diffuse_component: f32,
    pub specular_component: f32,
    pub shine: f32,
    pub reflectance: f32,
    pub transmittance: f32,
    pub refraction_index: f32
}

impl Surface for Phong {
    fn get_visible_color(&self,
            normal: &Vector,
            view: &Vector,
            light_direction: &Vector,
            light_color: &Color) -> Color {

        let ndv = vector_math::dot(normal, view);
        if ndv > 0.0 {
            // Viewing a back face - nothing visible
            return Color {r: 0.0, g: 0.0, b: 0.0};
        }

        let reflected_view = view - 2.0 * ndv * normal;

        let ldr = vector_math::dot(&reflected_view, light_direction);
        let specular_contrib = self.specular_component * ldr.powf(self.shine);

        let ndl = vector_math::dot(normal, light_direction);
        let diffuse_contrib = self.diffuse_component * ndl;

        Color {
            r: light_color.r *
                (specular_contrib + diffuse_contrib * self.color.r),
            g: light_color.g *
                (specular_contrib + diffuse_contrib * self.color.g),
            b: light_color.b *
                (specular_contrib + diffuse_contrib * self.color.b)
        }
    }
}