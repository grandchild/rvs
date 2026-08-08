#![allow(dead_code)]

pub trait RenderContext {}

pub trait Effect {}
pub trait GpuEffect {}

// Change effect's state by handling parameter changes.
pub trait ChangeEffect {}

pub trait GuiInterface {}
