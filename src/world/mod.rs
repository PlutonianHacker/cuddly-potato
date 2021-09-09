use bevy::prelude::*;

#[derive(Debug)]
pub enum TerrainType {
    GRASS
}

pub struct UnitRef {
    pub unit_type: Option<std::string::String>,
    pub id: Entity,
}

pub struct Cell {
    pub x: i64,
    pub y: i64,
    pub is_empty: bool,
    pub unit: Option<UnitRef>,
}

#[derive(Debug)]
pub struct Eviroment;

impl Plugin for Eviroment {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_map.system());
    }
}

fn init_map(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    for x in 0..32 {
        for y in 0..16 {
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(0.18, 0.8, 0.44).into()),
                    transform: Transform::from_xyz(
                        (x as f32 - 15.5) as f32 * 32.0,
                        (y - 5) as f32 * 32.0,
                        0.0,
                    ),
                    sprite: Sprite::new(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                })
                .insert(TerrainType::GRASS)
                .insert(Cell {
                    x,
                    y,
                    is_empty: true,
                    unit: None,
                });
        }
    }
}
