#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn boost_color(r: u8, g: u8, b: u8) -> Rgb {
    let factor = 1.3;
    let avg = (r as f32 + g as f32 + b as f32) / 3.0;
    Rgb {
        r: ((r as f32 - avg) * factor + avg).clamp(0.0, 255.0) as u8,
        g: ((g as f32 - avg) * factor + avg).clamp(0.0, 255.0) as u8,
        b: ((b as f32 - avg) * factor + avg).clamp(0.0, 255.0) as u8,
    }
}
