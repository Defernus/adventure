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
            scale: 0.1,
            noise_threshold: 0.6,
            simplex: noise::OpenSimplex::new(),
        }
    }

    fn get_level_val(&self, pos: Vec3<f64>) -> f64 {
        let mut noise_v = pos.y
            + (self
                .simplex
                .get([pos.x * 0.456 * self.scale, pos.z * 0.456 * self.scale])
                + 1.)
                * 10.
            + (self.simplex.get([pos.x, pos.z]) + 1.) * 0.001;

        noise_v -= self.noise_threshold;
        noise_v /= 100. / self.scale;

        return -noise_v;
    }

    fn get_cliffs_val(&self, pos: Vec3<f64>) -> f64 {
        let mut noise_v =
            self.simplex
                .get([pos.x * self.scale, pos.y * self.scale, pos.z * self.scale]);
        noise_v += 1.0;
        noise_v /= 2.0;

        noise_v -= (self.simplex.get([
            pos.x * 0.156 * self.scale,
            pos.y * 0.156 * self.scale,
            pos.z * 0.156 * self.scale,
        ]) + 1.)
            / 2.
            * self.noise_threshold;
        noise_v *= 10. * self.scale;

        return noise_v;
    }

    fn randomize_color(&self, pos: Vec3<f64>, color: [f32; 3], factor: f64) -> [f32; 3] {
        let dr = self.simplex.get([pos.x / 2.3, pos.y / 2.3, pos.z / 2.3]) * factor;
        let dg = self.simplex.get([-pos.x / 2.3, pos.y / 2.3, pos.z / 2.3]) * factor;
        let db = self.simplex.get([pos.x / 2.3, -pos.y / 2.3, pos.z / 2.3]) * factor;
        [
            (color[0] + dr as f32).max(0.).min(1.),
            (color[1] + dg as f32).max(0.).min(1.),
            (color[2] + db as f32).max(0.).min(1.),
        ]
    }

    pub fn generate_voxel(&self, pos: Vec3<f64>) -> Voxel {
        let mut color = [0.4; 3];

        let level = self.get_level_val(pos);
        if level < 0.03 * self.scale {
            color = [0.2, 0.7, 0.3];
        }

        let mut noise_v = self.get_cliffs_val(pos);

        noise_v = noise_v.min(level);

        return Voxel {
            color: self.randomize_color(pos, color, 0.05),
            value: noise_v as f32,
        };
    }
}
