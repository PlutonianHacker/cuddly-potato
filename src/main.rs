use bevy::{prelude::*, render::pass::ClearColor};

mod world;
mod units;
mod hud;

use units::resources;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(world::Eviroment)
        .add_plugin(hud::UIPlugin)
        .add_plugin(units::UnitSystem)
        .insert_resource(WindowDescriptor {
            title: "Hello World".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .init_resource::<resources::Units>()
        .init_resource::<resources::Resources>()
        .run();
}