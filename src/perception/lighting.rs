use crate::rng;
use rltk::RGB;
use std::cmp;

/// Something that gives off light.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightSource {
    /// The color of light produced.
    pub color: RGB,
    /// Radius of lighted area.
    pub radius: i32,
    /// The intensity of the light produced.
    pub intensity: i32,
}

/// Something that gives off light.
impl LightSource {
    /// Constructor.
    pub fn new(color: RGB, radius: i32, intensity: i32) -> LightSource {
        LightSource {
            color: color,
            radius: radius,
            intensity: intensity,
        }
    }

    /// Compute intensity of light at a distance.
    pub fn intensity_at(&self, x: i32, y: i32, x2: i32, y2: i32) -> i32 {
        let dx = (x2 - x).abs();
        let dy = (y2 - y).abs();
        if dx > self.radius || dy > self.radius {
            0
        } else {
            let distance = ((dx as f64).powi(2) + (dy as f64).powi(2)).sqrt();
            let coefficient = -(self.intensity as f64) / (self.radius as f64);
            let result: i32 = (self.intensity as f64 + distance * coefficient) as i32;
            let result = cmp::max(result, 0);
            result
        }
    }

    /// Transform a color at a specified distance.
    pub fn transform_color_at(&self, color: RGB, x: i32, y: i32, x2: i32, y2: i32) -> RGB {
        let intensity = self.intensity_at(x, y, x2, y2);
        let multiplier = intensity as f64 / 512 as f64;
        let red = color.r;
        let green = color.g;
        let blue = color.b;
        let r_diff = (self.color.r as i32 - red as i32).abs();
        let g_diff = (self.color.g as i32 - green as i32).abs();
        let b_diff = (self.color.b as i32 - blue as i32).abs();
        let new_r = (red as f64 + (r_diff as f64 * multiplier)) as u8;
        let new_g = (green as f64 + (g_diff as f64 * multiplier)) as u8;
        let new_b = (blue as f64 + (b_diff as f64 * multiplier)) as u8;
        RGB::from_u8(new_r, new_g, new_b)
    }
}

/// A factory.
#[derive(Clone, Copy, Debug)]
pub enum Factory {
    /// A candle provides very little light.
    Candle,
    /// A torch provides more and stronger light.
    Torch,
    /// A patch of phosphorescent moss.
    Moss,
    /// A completely random light source.
    Random,
}

/// A factory.
impl Factory {
    /// Creates a light source.
    pub fn create(self) -> LightSource {
        match self {
            Factory::Candle => LightSource::new(RGB::from_u8(255, 127, 255), 6, 64),
            Factory::Torch => LightSource::new(RGB::from_u8(255, 127, 0), 10, 96),
            Factory::Moss => LightSource::new(RGB::from_u8(173, 223, 173), 5, 32),
            Factory::Random => LightSource::new(
                RGB::from_u8(
                    rng::range(0, 5) as u8 * 60,
                    rng::range(0, 5) as u8 * 60,
                    rng::range(0, 5) as u8 * 60,
                ),
                rng::range(10, 15),
                rng::range(128, 255),
            ),
        }
    }
}
