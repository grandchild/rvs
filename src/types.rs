#![allow(dead_code)]

type ScreenCoordinate = u16;

type ColorIntensity = u8;
type Color = [ColorIntensity; 4];

type AvsInt = u32;
type AvsFloat = f32;

pub struct RenderContext {
    width: ScreenCoordinate,
    height: ScreenCoordinate,
    framebuffer: Vec<Vec<Color>>,
}

// Effect structure
pub struct Effect {
    inner: EffectInner,
    name: String,
    description: String,
    is_creatable: bool,
}

pub enum EffectInner {
    Single(SingleEffect),
    WithChildrenEffects(MetaEffect),
}

pub struct SingleEffect {
    effect_type: EffectType,
    parameters: Vec<EffectParameter>,
}

pub struct MetaEffect {
    parametes: Vec<EffectParameter>,
    children: Vec<Effect>,
}

pub enum EffectType {
    Render,
    Trans,
    Misc,
}

pub struct EffectParameter {
    is_global: bool,
    parameter: ParameterComponent,
    label: String,
}

// not yet complete
pub enum ParameterComponent {
    ParamBool(bool),
    ParamInt {
        value: AvsInt,
        min: AvsInt,
        max: AvsInt,
    },
    ParamFloat {
        value: AvsFloat,
        min: AvsFloat,
        max: AvsFloat,
    },
    ParamString(String),
    ParamSelect {
        selectables: Vec<String>,
    },
    ParameterComposit(Vec<ParameterComponent>),
}
