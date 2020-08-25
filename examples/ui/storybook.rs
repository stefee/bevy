use bevy::prelude::*;

fn main() {
    env_logger::init();

    App::build()
        .add_resource(bevy::window::WindowDescriptor {
            width: 1920,
            height: 1080,
            title: "Bevy UI Storybook Prototype".to_string(),
            ..Default::default()
        })
        .add_resource(bevy::render::pass::ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_default_plugins()
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // ui camera
        .spawn(UiCameraComponents::default())
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                align_items: AlignItems::Stretch,
                ..Default::default()
            },
            material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeComponents {
                    style: Style {
                        // size: Size::new(Val::Auto, Val::Auto),
                        flex_basis: Val::Percent(50.0),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    ..Default::default()
                })
                .spawn(NodeComponents {
                    style: Style {
                        // size: Size::new(Val::Auto, Val::Auto),
                        flex_basis: Val::Percent(50.0),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                    ..Default::default()
                });
        });
}
