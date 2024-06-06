use super::*;

pub struct DefaultFalseTermPlugins;

impl Plugin for DefaultFalseTermPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_ftbuilder);
    }
}

#[derive(Component, Debug, Clone)]
pub struct FTBuilder {
    contents: Vec<NodeBundle>,
    text_height: f32,
    padding: f32,
}

impl Default for FTBuilder {
    fn default() -> Self {
        Self {
            contents: Vec::new(),
            text_height: 12.,
            padding: 1.,
        }
    }
}

impl FTBuilder {
    pub fn add_horizontal_bar() {}
}

impl FTBuilder {
    fn absolute_height(&self, window: &Window) -> f32 {
        window.resolution.height()
    }

    fn absolute_width(&self, window: &Window) -> f32 {
        window.resolution.width()
    }
}

fn sync_ftbuilder(mut q_primary: Query<(&Window, &FTBuilder, &mut Children), Changed<FTBuilder>>) {
    for (w, ftb, ch) in q_primary.iter_mut() {}
}
