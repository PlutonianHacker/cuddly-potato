use bevy::prelude::*;

use crate::units::resources;

pub struct ViewPanel;
pub struct SpawnUnit(pub String);

pub struct MouseClick(pub bool);

#[derive(Debug)]
pub struct Unit {
    pub unit_type: std::string::String,
}

/*pub struct UnitRef {
    pub unit_type: Option<std::string::String>,
    pub id: Entity,
}*/

pub struct Mode {
    pub build: bool,
    pub view: bool,
}

pub struct SelectedUnitType {
    pub unit_type: Option<std::string::String>,
}

pub struct ViewEntityEvent(pub Entity);

fn mouse_input(buttons: Res<Input<MouseButton>>, mut ev: EventWriter<MouseClick>) {
    if buttons.just_pressed(MouseButton::Left) {
        ev.send(MouseClick(true));
    }
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &Children, &Unit)>,
    mut spawn_unit_query: Query<(&Interaction, &SpawnUnit)>,
    mut game_mode: ResMut<Mode>,
    mut selected_unit_type: ResMut<SelectedUnitType>,
    mut spawn_unit: EventWriter<SpawnUnit>,
) {
    for (interaction, _, unit) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                game_mode.view = false;
                game_mode.build = true;
                selected_unit_type.unit_type = Some(unit.unit_type.clone());
            }
            _ => {}
        }
    }
    for (interaction, _) in spawn_unit_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                spawn_unit.send(SpawnUnit("peasant".to_string()));
            }
            _ => {}
        }
    }
}

fn setup_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    units: Res<resources::Units>,
) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(80.0)),
                margin: Rect {
                    bottom: Val::Px(200.0),
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    top: Val::Auto,
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.35, 0.35, 0.35).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Peasant",
                    TextStyle {
                        font: asset_server.load("./fonts/OpenSans-Bold.ttf"),
                        font_size: 24.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(SpawnUnit("Peasant".to_string()));
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(90.0), Val::Percent(20.0)),
                justify_content: JustifyContent::SpaceBetween,
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Percent(80.0),
                    bottom: Val::Px(0.0),
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.75, 0.75, 0.75).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(20.0), Val::Percent(120.0)),
                        margin: Rect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Px(10.0),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.35, 0.35, 0.35).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Hello, World",
                                TextStyle {
                                    font: asset_server.load("./fonts/OpenSans-Bold.ttf"),
                                    font_size: 24.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(ViewPanel);
                });
            for i in 0..units.all_units.buildings.len() {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(12.0), Val::Percent(80.0)),
                            margin: Rect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: materials.add(Color::rgb(0.35, 0.35, 0.35).into()),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: format!("{}\n", units.all_units.buildings[i].unit_name),
                                    style: TextStyle {
                                        font: asset_server.load("./fonts/OpenSans-Bold.ttf"),
                                        font_size: 24.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                }],
                                ..Default::default()
                            },

                            ..Default::default()
                        });
                    })
                    .insert(Unit {
                        unit_type: units.all_units.buildings[i].clone().unit_name,
                    });
            }
        });
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui.system());
        app.add_system(button_system.system());
        app.add_system(mouse_input.system());
        app.insert_resource(Mode {
            build: false,
            view: true,
        });
        app.insert_resource(SelectedUnitType { unit_type: None });
        app.add_event::<ViewEntityEvent>();
        app.add_event::<SpawnUnit>();
        app.add_event::<MouseClick>();
    }
}
