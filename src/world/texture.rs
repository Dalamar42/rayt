use data::colour::Colour;
use data::vector::Vector;
use rand::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Texture {
    Constant {
        colour: Colour,
    },
    Checker {
        even: Box<Texture>,
        odd: Box<Texture>,
    },
    Noise {
        ran: Vec<Colour>,
        perm_x: Vec<usize>,
        perm_y: Vec<usize>,
        perm_z: Vec<usize>,
        scale: f64,
    },
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: &Vector) -> Colour {
        match self {
            Texture::Constant { colour } => colour.clone(),
            Texture::Checker { odd, even } => checker_texture(&odd, &even, u, v, &p),
            Texture::Noise {
                ran,
                perm_x,
                perm_y,
                perm_z,
                scale,
            } => {
                let base = Colour::new(1.0, 1.0, 1.0);
                let noise = turbulence(&ran, &perm_x, &perm_y, &perm_z, &p, 7);
                let mult = 0.5 * (1.0 + f64::sin(scale * p.z() + 10.0 * noise));

                mult * base
            },
        }
    }
}

fn checker_texture(odd: &Texture, even: &Texture, u: f64, v: f64, p: &Vector) -> Colour {
    let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
    if sines < 0.0 {
        odd.value(u, v, &p)
    } else {
        even.value(u, v, &p)
    }
}

fn turbulence(
    ran: &Vec<Colour>,
    perm_x: &Vec<usize>,
    perm_y: &Vec<usize>,
    perm_z: &Vec<usize>,
    p: &Vector,
    depth: u8,
) -> f64 {
    let mut accum = 0.0;
    let mut weight = 1.0;
    let mut p = p.clone();

    for i in 0..depth {
        accum += weight * perlin_noise(
            ran, perm_x, perm_y, perm_z, &p,
        );
        weight *= 0.5;
        p = p * 2.0;
    }

    accum.abs()
}

fn perlin_noise(
    ran: &Vec<Colour>,
    perm_x: &Vec<usize>,
    perm_y: &Vec<usize>,
    perm_z: &Vec<usize>,
    p: &Vector,
) -> f64 {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();

    let i = p.x().floor() as i64;
    let j = p.y().floor() as i64;
    let k = p.z().floor() as i64;

    let mut c: [[[Colour; 2]; 2]; 2] = [[[Colour::new(0.0, 0.0, 0.0); 2]; 2]; 2];
    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                let idx_i = (i + di) & 255;
                let idx_j = (j + dj) & 255;
                let idx_k = (k + dk) & 255;

                let idx = perm_x[idx_i as usize] ^ perm_y[idx_j as usize] ^ perm_z[idx_k as usize];

                c[di as usize][dj as usize][dk as usize] = ran[idx];
            }
        }
    }

    perlin_interpolation(c, u, v, w)
}

fn perlin_interpolation(c: [[[Colour; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let ran = &c[i][j][k];

                let i = i as f64;
                let j = j as f64;
                let k = k as f64;

                let weight = Colour::new(u - i, v - j, w - k);

                accum += Colour::dot(&ran, &weight)
                    * (i * uu + (1.0 - i) * (1.0 - uu))
                    * (j * vv + (1.0 - j) * (1.0 - vv))
                    * (k * ww + (1.0 - k) * (1.0 - ww));
            }
        }
    }

    accum
}

pub fn build_noise_texture(scale: f64) -> Texture {
    Texture::Noise {
        ran: perlin_generate().to_vec(),
        perm_x: perlin_generate_perm().to_vec(),
        perm_y: perlin_generate_perm().to_vec(),
        perm_z: perlin_generate_perm().to_vec(),
        scale,
    }
}

fn perlin_generate() -> [Colour; 256] {
    let mut rng = rand::thread_rng();

    let mut p = [Colour::new(0.0, 0.0, 0.0); 256];
    for i in 0..256 {
        p[i] = Colour::new(
            -1.0 + 2.0 * rng.gen::<f64>(),
            -1.0 + 2.0 * rng.gen::<f64>(),
            -1.0 + 2.0 * rng.gen::<f64>(),
        )
        .unit_vector();
    }

    p
}

fn permute(p: &mut [usize; 256]) {
    let mut rng = rand::thread_rng();

    for i in (0..256).rev() {
        let target = rng.gen::<usize>() % (i + 1);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn perlin_generate_perm() -> [usize; 256] {
    let mut p: [usize; 256] = [0; 256];
    for i in 0..256 {
        p[i] = i;
    }
    permute(&mut p);
    p
}
