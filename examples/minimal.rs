use bevy::prelude::*;
use bevy_falseterm::core::{DefaultFalseTermPlugins, FTBuilder};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultFalseTermPlugins)
        .add_systems(Startup, startup);
}

pub fn startup(mut co: Commands) {
    co.spawn((
        Window {
            title: String::from("Auxiliary Window"),
            focused: false,
            ..Default::default()
        },
        FTBuilder::default(),
    ));
    co.spawn((
        Window {
            title: String::from("Auxiliary Window"),
            focused: false,
            ..Default::default()
        },
        FTBuilder::default(),
    ));
}
