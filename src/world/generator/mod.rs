use noise::NoiseFn;

use crate::vec::Vec3;

use super::voxel::Voxel;

pub struct Generator {
    scale: f64,
    noise_threshold: f64,
    simplex: noise::OpenSimplex,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            scale: 0.01,
            noise_threshold: 0.4,
            simplex: noise::OpenSimplex::new(),
        }
    }

    fn get_level_val(&self, pos: Vec3<f64>) -> f64 {
        pos.y
            + (self
                .simplex
                .get([pos.x * 0.456 * self.scale, pos.z * 0.456 * self.scale])
                + 1.)
                * 100.
            + (self.simplex.get([pos.x, pos.z]) + 1.) * 0.001
    }

    fn get_cliffs_val(&self, pos: Vec3<f64>) -> f64 {
        let mut noise_v =
            self.simplex
                .get([pos.x * self.scale, pos.y * self.scale, pos.z * self.scale]);
        noise_v += 1.0;
        noise_v /= 2.0;

        return noise_v;
    }

    pub fn generate_voxel(&self, pos: Vec3<f64>) -> Voxel {
        let mut color = [0.4; 3];

        let level = self.get_level_val(pos);
        if level > -4.0 {
            color = [0.2, 0.7, 0.3];
        }

        let mut noise_v = self.get_cliffs_val(pos);

        noise_v = (1.0 - level);
        noise_v -= self.noise_threshold;
        noise_v /= 100.;

        return Voxel {
            color,
            value: noise_v as f32,
        };
    }
}
