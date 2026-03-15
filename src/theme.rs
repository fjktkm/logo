#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub name: &'static str,
    pub background: Color,
    pub gradient_start: Color,
    pub gradient_end: Color,
    pub bg_noise_intensity: f64,
    pub shape_noise_intensity: f64,
    pub noise_seed: u64,
}

pub const DARK: Theme = Theme {
    name: "dark",
    background: Color::new(1.0 / 20.0, 1.0 / 15.0, 3.0 / 20.0),
    gradient_start: Color::new(0.0, 1.0 / 6.0, 1.0),
    gradient_end: Color::new(3.0 / 5.0, 1.0, 1.0),
    bg_noise_intensity: 1.0 / 24.0,
    shape_noise_intensity: 1.0 / 24.0,
    noise_seed: 42,
};

pub const LIGHT: Theme = Theme {
    name: "light",
    background: Color::new(1.0, 1.0, 1.0),
    gradient_start: Color::new(0.0, 1.0 / 6.0, 1.0),
    gradient_end: Color::new(3.0 / 5.0, 1.0, 1.0),
    bg_noise_intensity: 0.0,
    shape_noise_intensity: 1.0 / 24.0,
    noise_seed: 42,
};

pub const ALL_THEMES: &[Theme] = &[DARK, LIGHT];
