use super::*;

pub struct DefaultFalseTermPlugins;

impl Plugin for DefaultFalseTermPlugins {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Debug, Clone, Copy)]
pub struct FTBuilder {}

impl Default for FTBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl FTBuilder {
    pub fn absolute_height(&self, window: &Window) -> f32 {
        window.resolution.height()
    }

    pub fn absolute_width(&self, window: &Window) -> f32 {
        window.resolution.width()
    }
}
