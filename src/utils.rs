use bevy::math::Vec3;
use rand::Rng;

pub fn random_vector() -> Vec3 {
    let mut rng = rand::thread_rng();

    Vec3 {
        x: rng.gen_range(-1.0..=1.0),
        y: rng.gen_range(-1.0..=1.0),
        z: 0.0,
    }
    .normalize()
}

pub fn random_mag_from_range(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..=max)
}
