use bevy::color::{Color, LinearRgba};

pub struct ColorPalette;

impl ColorPalette {
    pub const       RED:Color = Color::LinearRgba(LinearRgba {red: 1.000, green: 0.000, blue: 0.000, alpha: 1f32});   //rgb01(1.000, 0.000, 0.000)
    pub const     WHITE:Color = Color::LinearRgba(LinearRgba {red: 0.933, green: 0.933, blue: 0.933, alpha: 1f32});   //rgb01(0.933, 0.933, 0.933)
    pub const      GRAY:Color = Color::LinearRgba(LinearRgba {red: 0.408, green: 0.427, blue: 0.463, alpha: 1f32});   //rgb01(0.408, 0.427, 0.463)
    pub const  DARKGRAY:Color = Color::LinearRgba(LinearRgba {red: 0.216, green: 0.227, blue: 0.251, alpha: 1f32});   //rgb01(0.216, 0.227, 0.251)
    pub const    ORANGE:Color = Color::LinearRgba(LinearRgba {red: 0.863, green: 0.373, blue: 0.000, alpha: 1f32});   //rgb01(0.863, 0.373, 0.000)
}
