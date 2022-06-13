use noise::NoiseFn;

use crate::vec::Vec3;

use super::voxel::Voxel;

pub struct Generator {
    noise_scale: f64,
    noise_threshold: f64,
    simplex: noise::OpenSimplex,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            noise_scale: 0.03,
            noise_threshold: 0.3,
            simplex: noise::OpenSimplex::new(),
        }
    }

    fn get_level_val(&self, pos: Vec3<f64>) -> f64 {
        pos.y / 50. * (self.simplex.get([pos.x * 0.00456, pos.z * 0.00456]) + 1.)
    }

    fn get_cliffs_val(&self, pos: Vec3<f64>) -> f64 {
        let mut noise_v = self.simplex.get([
            pos.x * self.noise_scale,
            pos.y * self.noise_scale,
            pos.z * self.noise_scale,
        ]);
        noise_v += 1.0;
        noise_v /= 2.0;

        return noise_v;
    }

    pub fn generate_voxel(&self, pos: Vec3<f64>) -> Voxel {
        let mut color = [0.4; 3];

        let mut level = self.get_level_val(pos);

        let mut noise_v = self.get_cliffs_val(pos);

        noise_v = noise_v * (1.0 - level.min(1.0).max(0.0));
        noise_v -= self.noise_threshold;
        noise_v /= self.noise_scale;

        return Voxel {
            color,
            value: noise_v as f32,
        };
    }
}
