const GREEN_CHANNEL_SEED_SALT: u64 = 0x9e37_79b9_7f4a;
const RED_CHANNEL_SEED_SALT: u64 = 0x6c62_272e_07bb;
const SECOND_SAMPLE_SEED_SALT: u64 = 0xdead_beef_cafe;

fn uniform_sample(x: usize, y: usize, seed: u64) -> f64 {
    let mut hash = seed;
    hash ^= (x as u64).wrapping_mul(0x9e3779b97f4a7c15);
    hash ^= (y as u64).wrapping_mul(0x6c62272e07bb0142);
    hash ^= hash >> 30;
    hash = hash.wrapping_mul(0xbf58476d1ce4e5b9);
    hash ^= hash >> 27;
    hash = hash.wrapping_mul(0x94d049bb133111eb);
    hash ^= hash >> 31;
    (hash >> 11) as f64 / (1u64 << 53) as f64
}

fn gaussian_sample(x: usize, y: usize, seed: u64) -> f64 {
    let radius = (-2.0 * (1.0 - uniform_sample(x, y, seed)).ln()).sqrt();
    let angle = std::f64::consts::TAU * uniform_sample(x, y, seed ^ SECOND_SAMPLE_SEED_SALT);
    radius * angle.cos()
}

fn add_noise(channel: u8, delta: f64) -> u8 {
    ((channel as f64 / 255.0 + delta).clamp(0.0, 1.0) * 255.0) as u8
}

pub fn apply_noise_argb32(data: &mut [u8], size: u32, seed: u64, intensity: f64, stride: usize) {
    if intensity <= 0.0 {
        return;
    }

    let size = size as usize;
    for (y, row) in data.chunks_exact_mut(stride).take(size).enumerate() {
        for (x, pixel) in row.chunks_exact_mut(4).take(size).enumerate() {
            if pixel[3] == 0 {
                continue;
            }

            let blue_noise = gaussian_sample(x, y, seed) * intensity;
            let green_noise = gaussian_sample(x, y, seed ^ GREEN_CHANNEL_SEED_SALT) * intensity;
            let red_noise = gaussian_sample(x, y, seed ^ RED_CHANNEL_SEED_SALT) * intensity;

            pixel[0] = add_noise(pixel[0], blue_noise);
            pixel[1] = add_noise(pixel[1], green_noise);
            pixel[2] = add_noise(pixel[2], red_noise);
        }
    }
}
