use bevy::prelude::*;
use bevy_falseterm::core::{DefaultFalseTermPlugins, FTBuilder};
//use bevy_pancam::{PanCam, PanCamPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Primary Window".into(),
                ..Default::default()
            }),
            ..default()
        }))
        //.add_plugins(PanCamPlugin)
        .add_plugins(DefaultFalseTermPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, debugging)
        .run();
}

fn custom_falseterm() -> FTBuilder {
    let ftb = FTBuilder::default();
    let node1 = NodeBundle {
        //move to FTBuilder function.
        node: todo!(),
        style: todo!(),
        background_color: todo!(),
        z_index: ZIndex::Global(i32::MAX),
        ..Default::default()
    };

    ftb
}

pub fn startup(mut co: Commands) {
    let id1 = co
        .spawn((
            Window {
                title: String::from("Auxiliary Window <1>"),
                focused: false,
                ..Default::default()
            },
            custom_falseterm(),
        ))
        .id();
    let id2 = co
        .spawn((
            Window {
                title: String::from("Auxiliary Window <2>"),
                focused: false,
                ..Default::default()
            },
            custom_falseterm(),
        ))
        .id();

    co.spawn((
        Camera2dBundle {
            camera: Camera {
                target: bevy::render::camera::RenderTarget::Window(bevy_window::WindowRef::Entity(
                    id1,
                )),
                ..default()
            },
            ..default()
        },
        //PanCam::default(),
    ));
    co.spawn((
        Camera2dBundle {
            camera: Camera {
                target: bevy::render::camera::RenderTarget::Window(bevy_window::WindowRef::Entity(
                    id2,
                )),
                ..default()
            },
            ..default()
        },
        //PanCam::default(),
    ));
}

pub fn debugging(a: Query<(Entity, &Window, &FTBuilder)>) {
    for (e, w, ftb) in a.iter() {
        let name = &w.name;
    }
}
