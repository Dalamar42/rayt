use data::colour::Colour;
use data::vector::Vector;
use rand::prelude::*;

const RAN_SIZE: usize = 256;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoiseConfig {
    ran: Vec<Colour>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

pub fn perlin_turbulence(config: &NoiseConfig, point: &Vector, depth: u8) -> f64 {
    let mut accum = 0.0;
    let mut weight = 1.0;
    let mut point = *point;

    for _ in 0..depth {
        accum += weight * perlin_noise(config, &point);
        weight *= 0.5;
        point = point * 2.0;
    }

    accum.abs()
}

fn perlin_noise(config: &NoiseConfig, point: &Vector) -> f64 {
    let intra_pixel_distance_u = point.x() - point.x().floor();
    let intra_pixel_distance_v = point.y() - point.y().floor();
    let intra_pixel_distance_w = point.z() - point.z().floor();

    let corner_i = point.x().floor() as i64;
    let corner_j = point.y().floor() as i64;
    let corner_k = point.z().floor() as i64;

    let mut ran_matrix: [[[Colour; 2]; 2]; 2] = [[[Colour::new(0.0, 0.0, 0.0); 2]; 2]; 2];
    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                let idx_i = (corner_i + di) & 255;
                let idx_j = (corner_j + dj) & 255;
                let idx_k = (corner_k + dk) & 255;

                let idx = config.perm_x[idx_i as usize]
                    ^ config.perm_y[idx_j as usize]
                    ^ config.perm_z[idx_k as usize];

                ran_matrix[di as usize][dj as usize][dk as usize] = config.ran[idx];
            }
        }
    }

    perlin_interpolation(
        ran_matrix,
        intra_pixel_distance_u,
        intra_pixel_distance_v,
        intra_pixel_distance_w,
    )
}

fn perlin_interpolation(
    ran_matrix: [[[Colour; 2]; 2]; 2],
    intra_pixel_distance_u: f64,
    intra_pixel_distance_v: f64,
    intra_pixel_distance_w: f64,
) -> f64 {
    // Compute hermite cubic to eliminate Mach bands
    let uu = intra_pixel_distance_u * intra_pixel_distance_u * (3.0 - 2.0 * intra_pixel_distance_u);
    let vv = intra_pixel_distance_v * intra_pixel_distance_v * (3.0 - 2.0 * intra_pixel_distance_v);
    let ww = intra_pixel_distance_w * intra_pixel_distance_w * (3.0 - 2.0 * intra_pixel_distance_w);

    let mut accum = 0.0;

    for (i, i_axis) in ran_matrix.iter().enumerate() {
        for (j, j_axis) in i_axis.iter().enumerate() {
            for (k, ran) in j_axis.iter().enumerate() {
                let i = i as f64;
                let j = j as f64;
                let k = k as f64;

                let weight = Colour::new(
                    intra_pixel_distance_u - i,
                    intra_pixel_distance_v - j,
                    intra_pixel_distance_w - k,
                );

                accum += Colour::dot(&ran, &weight)
                    * (i * uu + (1.0 - i) * (1.0 - uu))
                    * (j * vv + (1.0 - j) * (1.0 - vv))
                    * (k * ww + (1.0 - k) * (1.0 - ww));
            }
        }
    }

    accum
}

pub fn build_noise_config() -> NoiseConfig {
    NoiseConfig {
        ran: perlin_generate_ran().to_vec(),
        perm_x: perlin_generate_perm().to_vec(),
        perm_y: perlin_generate_perm().to_vec(),
        perm_z: perlin_generate_perm().to_vec(),
    }
}

fn perlin_generate_ran() -> [Colour; RAN_SIZE] {
    let mut rng = rand::thread_rng();

    let mut ran = [Colour::new(0.0, 0.0, 0.0); RAN_SIZE];
    for item in ran.iter_mut() {
        *item = Colour::new(
            -1.0 + 2.0 * rng.gen::<f64>(),
            -1.0 + 2.0 * rng.gen::<f64>(),
            -1.0 + 2.0 * rng.gen::<f64>(),
        )
        .unit_vector();
    }

    ran
}

fn permute(perm: &mut [usize; RAN_SIZE]) {
    let mut rng = rand::thread_rng();

    for i in (0..RAN_SIZE).rev() {
        let target = rng.gen::<usize>() % (i + 1);
        perm.swap(i, target);
    }
}

fn perlin_generate_perm() -> [usize; RAN_SIZE] {
    let mut perm = [0; RAN_SIZE];
    for (i, item) in perm.iter_mut().enumerate() {
        *item = i;
    }
    permute(&mut perm);
    perm
}
