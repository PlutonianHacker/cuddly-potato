use bevy::{prelude::*, render::pass::ClearColor};

mod units;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Hello World".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(Mode {
            build: false,
            view: true,
        })
        .insert_resource(SelectedUnitType { unit_type: None })
        .init_resource::<Resources>()
        .init_resource::<units::Units>()
        .add_event::<MouseClick>()
        .add_event::<ViewEntityEvent>()
        .add_startup_system(setup.system())
        .add_startup_system(test_units.system())
        .add_system(mouse_input.system())
        .add_system(select_entity.system())
        .add_system(handle_selected_entity.system())
        .add_system(view_selected_entity.system())
        .add_system(timer_system.system())
        .add_system(production_system.system())
        .add_plugin(UIPlugin)
        .run();
}

// construct buildings
// maintain villagers
// gather resources
// explore territory
// train warriors for battle
// build infrastructure and defenses
// unlock technologies

fn test_units(units: Res<units::Units>) {
    let name = "House";
    let mut iter = units
        .all_units
        .buildings
        .iter()
        .filter(|x| x.unit_name == name);
    println!("{:?}", iter.next());
    for i in 0..3 {
        println!("{:?}", units.all_units.buildings[i].unit_name);
    }
}

struct MouseClick(bool);

struct ViewEntityEvent(Entity);

#[derive(Debug)]
struct Select;

#[derive(Debug)]
struct ViewUnit;

/*
#[derive(Debug, Clone, PartialEq)]
enum String {
    HOUSE,
    MILL,
    HALL,
    BARRACKS,
    GRANARY,
    MINE,
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            String::HOUSE => write!(f, "House"),
            String::MILL => write!(f, "Mill"),
            String::HALL => write!(f, "Town Hall"),
            String::BARRACKS => write!(f, "Barracks"),
            String::GRANARY => write!(f, "Granary"),
            String::MINE => write!(f, "Gold Mine"),
        }
    }
}

impl String {
    fn cost(&self) -> i32 {
        match *self {
            String::HOUSE => 150,    // units -> commoners (e.g. pheasants)
            String::BARRACKS => 215, // units -> soliders
            String::MILL => 200,     // gold or units -> fields
            String::HALL => 500,     // gold or units -> elite units
            String::GRANARY => 180,  // gold
            String::MINE => 350,     // gold
        }
    }
    fn cooldown(&self) -> f32 {
        match *self {
            String::MILL => 20.0,
            String::HALL => 35.0,
            String::GRANARY => 15.0,
            String::MINE => 25.0,
            _ => 0.0,
        }
    }
    fn production(&self) -> i32 {
        match *self {
            String::MILL => 50,
            String::HALL => 100,
            String::GRANARY => 25,
            String::MINE => 80,
            _ => 0,
        }
    }
}*/

#[derive(Debug)]
struct Unit {
    unit_type: std::string::String,
}

#[derive(Debug)]
struct Cost {
    cost: i32,
}

#[derive(Debug)]
struct Production {
    rate: f32,
    amount: i32,
    cooldown: CoolDown,
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

#[derive(Debug)]
struct UnitRef {
    unit_type: Option<std::string::String>,
    id: Entity,
}

#[derive(Debug)]
struct Cell {
    x: i64,
    y: i64,
    is_empty: bool,
    unit: Option<UnitRef>,
}

struct Map {
    data: Vec<Cell>,
}

#[derive(Debug)]
struct ViewPanel;

#[derive(Default)]
struct Mode {
    build: bool,
    view: bool,
}

#[derive(Default)]
struct SelectedUnitType {
    unit_type: Option<std::string::String>,
}

#[derive(Debug)]
struct Resources {
    gold: i32,
}

impl Resources {
    fn spend(&mut self, price: i32) {
        self.gold -= price;
    }
    fn can_spend(&self, price: i32) -> bool {
        if self.gold - price <= 0 {
            return false;
        } else {
            return true;
        }
    }
    fn add(&mut self, amount: i32) {
        self.gold += amount;
    }
}

impl FromWorld for Resources {
    fn from_world(_world: &mut World) -> Self {
        let start_val = 2000;
        Resources { gold: start_val }
    }
}
/*
struct Units {
    all_units: serde_json::Value,
}*/
/*
impl FromWorld for Units {
    fn
}*/

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut data: Vec<Cell> = Vec::new();
    for y in 0..15 {
        for x in 0..20 {
            data.push(Cell {
                x,
                y,
                is_empty: true,
                unit: None,
            })
        }
    }
    let map = Map { data };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    for cell in map.data {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.18, 0.8, 0.44).into()),
                transform: Transform::from_xyz(
                    (cell.x - 10) as f32 * 32.0,
                    (cell.y - 5) as f32 * 32.0,
                    0.0,
                ),
                sprite: Sprite::new(Vec2::new(32.0, 32.0)),
                ..Default::default()
            })
            .insert(Cell {
                x: cell.x,
                y: cell.y,
                is_empty: true,
                unit: None,
            });
    }
}

fn mouse_input(buttons: Res<Input<MouseButton>>, mut ev: EventWriter<MouseClick>) {
    if buttons.just_pressed(MouseButton::Left) {
        ev.send(MouseClick(true));
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
    mut resources: ResMut<Resources>,
    units: Res<units::Units>,
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
            /*if selected_unit_type.unit_type.clone().unwrap() == String::MINE {
                /*commands.entity(unit_id).insert(Production {
                    rate: 5.0,
                    amount: 10,
                    cooldown: CoolDown::new(2.5, true),
                });*/
            }*/
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

fn timer_system(time: Res<Time>, mut cooldown_timer: Query<(Entity, &mut CoolDown)>) {
    for (_, mut cooldown) in cooldown_timer.iter_mut() {
        if cooldown.timer.tick(time.delta()).just_finished() {
            //println!("Cooldown {:?} complete", entity);
        }
    }
}

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
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &Children, &Unit)>,
    mut game_mode: ResMut<Mode>,
    mut selected_unit_type: ResMut<SelectedUnitType>,
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
}

fn setup_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    units: Res<units::Units>,
) {
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

struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui.system());
        app.add_system(button_system.system());
    }
}
