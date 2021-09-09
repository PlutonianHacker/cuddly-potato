use bevy::prelude::*;

pub mod resources;

use crate::hud::{Mode, MouseClick, SelectedUnitType, SpawnUnit, Unit, ViewEntityEvent, ViewPanel};
use crate::world::{Cell, UnitRef};

#[derive(Debug)]
struct Select;

#[derive(Debug)]
struct Cost {
    cost: i32,
}

#[derive(Debug)]
struct CoolDown {
    timer: Timer,
}

impl CoolDown {
    fn new(time: f32, restart: bool) -> Self {
        CoolDown {
            timer: Timer::from_seconds(time, restart),
        }
    }
}

fn select_entity(
    mut commands: Commands,
    windows: Res<Windows>,
    game_mode: Res<Mode>,
    mut mouse_ev: EventReader<MouseClick>,
    mut view_unit_ev: EventWriter<ViewEntityEvent>,
    mut query: Query<(&mut Cell, Entity)>,
) {
    let window = windows.get_primary().unwrap();
    for _ in mouse_ev.iter() {
        if let Some(pos) = window.cursor_position() {
            let x = (pos.x / 32.0).round();
            let y = (pos.y / 32.0).round();
            for (cell, entity) in &mut query.iter_mut() {
                if cell.x + 10 == x as i64 && cell.y + 6 == y as i64 {
                    if game_mode.build {
                        commands.entity(entity).insert(Select);
                    }
                    if game_mode.view {
                        if let Some(_) = cell.unit {
                            view_unit_ev.send(ViewEntityEvent(cell.unit.as_ref().unwrap().id));
                        }
                    }
                }
            }
        }
    }
}

fn handle_selected_entity(
    mut commands: Commands,
    mut query: Query<(&mut Select, &mut Cell, Entity)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_mode: ResMut<Mode>,
    mut resources: ResMut<resources::Resources>,
    units: Res<resources::Units>,
    selected_unit_type: ResMut<SelectedUnitType>,
) {
    for (_, mut cell, entity) in &mut query.iter_mut() {
        let gold = resources.gold;
        let mut iter = units
            .all_units
            .buildings
            .iter()
            .filter(|x| x.unit_name == selected_unit_type.unit_type.clone().unwrap());
        let cost = iter.next().unwrap().cost;
        println!("gold: {} cost: {}", gold, cost);
        if cell.is_empty && resources.can_spend(cost) {
            let unit_id = commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(0.75, 0.75, 0.75).into()),
                    transform: Transform::from_xyz(
                        (cell.x - 10) as f32 * 32.0,
                        (cell.y - 5) as f32 * 32.0,
                        0.2,
                    ),
                    sprite: Sprite::new(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                })
                .insert(Unit {
                    unit_type: selected_unit_type.unit_type.clone().unwrap(),
                })
                .insert(Cost { cost: cost })
                .insert(CoolDown::new(5.0, true))
                .id();
            resources.spend(cost);
            cell.unit = Some(UnitRef {
                unit_type: Some(selected_unit_type.unit_type.clone().unwrap()),
                id: unit_id,
            });
            cell.is_empty = false;
        }
        commands.entity(entity).remove::<Select>();
        game_mode.build = false;
        game_mode.view = true;
    }
}

fn spawn_unit(
    mut _commands: Commands,
    mut ev_spawn: EventReader<SpawnUnit>,
    units: Res<resources::Units>,
) {
    for ev in ev_spawn.iter() {
        let mut iter = units.all_units.units.iter().filter(|x| x.unit_name == ev.0);
        println!("spawn unit: {:?}", iter.next().unwrap());
    }
}

fn view_selected_entity(
    mut view_entity_ev: EventReader<ViewEntityEvent>,
    unit_query: Query<(Entity, &Unit)>,
    entity: Query<(&Unit, &Cost)>,
    mut ui_query: Query<&mut Text, With<ViewPanel>>,
) {
    for ev in view_entity_ev.iter() {
        let unit = unit_query.get(ev.0).unwrap();
        for mut ui_entity in ui_query.iter_mut() {
            ui_entity.sections[0].value = format!("{}", entity.get(unit.0).unwrap().0.unit_type);
        }
    }
}

fn _timer_system(time: Res<Time>, mut cooldown_timer: Query<(Entity, &mut CoolDown)>) {
    for (_, mut cooldown) in cooldown_timer.iter_mut() {
        if cooldown.timer.tick(time.delta()).just_finished() {
            //println!("Cooldown {:?} complete", entity);
        }
    }
}

pub struct UnitSystem;

impl Plugin for UnitSystem {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(select_entity.system());
        app.add_system(handle_selected_entity.system());
        app.add_system(view_selected_entity.system());
        app.add_system(spawn_unit.system());
    }
}
/*
fn production_system(
    time: Res<Time>,
    mut res: ResMut<Resources>,
    mut query: Query<(Entity, &mut Production)>,
) {
    for (_, mut production) in query.iter_mut() {
        if production.cooldown.timer.tick(time.delta()).just_finished() {
            res.add(production.amount);
            println!("{:?}", res.gold);
        }
    }
}*/
