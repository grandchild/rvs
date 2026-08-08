#![allow(dead_code)]

type ScreenCoordinate = u16;

type ColorIntensity = u8;
type Color = [ColorIntensity; 4];

pub struct RenderContext {
    width: ScreenCoordinate,
    height: ScreenCoordinate,
    framebuffer: Vec<Vec<Color>>,
}

// ControlParameters
pub enum EffectParameter {
    Switch(bool),
    ByteSlider(u8),
}
