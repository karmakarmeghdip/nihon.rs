//! UI utility functions

use iced::Color;

/// Utility function to mix two colors
pub fn mix_colors(a: Color, b: Color, factor: f32) -> Color {
    let t = factor.clamp(0.0, 1.0);

    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}
