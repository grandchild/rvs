#![allow(dead_code)]

use crate::types::EffectParameter;

pub trait RenderContext {}

pub trait Effect {
    fn handle_parameter_update();
    fn parameter_info();
    fn render();
}

pub trait GpuEffect: Effect {
    fn into_shader();
    fn render_gpu();
}

// Change effect's state by handling parameter changes.
pub trait ChangeEffect {
    fn handle_parameter(param: &EffectParameter);
}

pub trait GuiInterface {}
