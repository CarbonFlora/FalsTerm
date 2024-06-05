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
