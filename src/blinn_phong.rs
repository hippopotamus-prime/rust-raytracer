use crate::color::Color;
use crate::render::Surface;
use crate::vector_math;
use crate::vector_math::Vector;

pub struct BlinnPhong {
    pub color: Color,
    pub diffuse_component: f32,
    pub specular_component: f32,
    pub shine: f32,
    pub reflectance: f32,
    pub transmittance: f32,
    pub refraction_index: f32
}

impl Surface for BlinnPhong {
    fn get_reflectance(&self) -> f32 {
        self.reflectance
    }

    fn get_transmittance(&self) -> f32 {
        self.transmittance
    }

    fn get_visible_color(&self,
            normal: &Vector,
            view: &Vector,
            light_direction: &Vector,
            light_color: &Color) -> Color {
        // Note - view & light have opposite directions here
        let half = (light_direction - view).normalized();

        let mut specular_contrib = 0.0;
        let ndh = vector_math::dot(normal, &half);
        if ndh > 0.0 {
            specular_contrib = self.specular_component * ndh.powf(self.shine);
        }

        let mut diffuse_contrib = 0.0;
        let ndl = vector_math::dot(normal, light_direction);
        if ndl > 0.0 {
            diffuse_contrib = self.diffuse_component * ndl;
        }

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
