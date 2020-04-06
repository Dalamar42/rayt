use data::vector::Vector;
use rand::distributions::Standard;
use rand::prelude::*;

pub fn uniform<T>() -> T
where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    rng.gen::<T>()
}

pub fn random_point_in_unit_sphere() -> Vector {
    let centre = Vector::new(1.0, 1.0, 1.0);

    loop {
        let point = 2.0 * Vector::new(uniform(), uniform(), uniform()) - centre;
        if point.len_squared() < 1.0 {
            return point;
        }
    }
}
