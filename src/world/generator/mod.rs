use noise::NoiseFn;

use crate::vec::Vec3;

use super::voxel::Voxel;

pub struct Generator {
    noise_scale: f64,
    noise_threshold: f32,
    simplex: noise::OpenSimplex,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            noise_scale: 0.03,
            noise_threshold: 0.4,
            simplex: noise::OpenSimplex::new(),
        }
    }

    pub fn generate_voxel(&self, pos: Vec3<f64>) -> Voxel {
        let mut noise_v = self.simplex.get([
            pos.x * self.noise_scale,
            pos.y * self.noise_scale,
            pos.z * self.noise_scale,
        ]) as f32;
        noise_v += 1.0;
        noise_v /= 2.0;

        noise_v *= 1.0 - (pos.y as f32 / 10.).min(1.0).max(0.0);
        noise_v -= self.noise_threshold;
        noise_v /= self.noise_scale as f32;

        if noise_v < 0. {
            return Voxel {
                id: 0,
                value: noise_v,
            };
        } else {
            return Voxel {
                id: 1,
                value: noise_v,
            };
        };
    }
}
