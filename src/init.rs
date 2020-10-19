use super::costs::*;
use super::resources::*;
use bevy::prelude::*;

pub struct ResPlugin;
impl Plugin for ResPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(World(Timer::from_seconds(0.1, true)))
            .add_resource(Costs::new())
            .add_resource(Energy::new())
            .add_resource(Batteries::new())
            .init_resource::<ButtonMaterials>()
            .add_startup_system(init.system())
            .add_system(buttons.system())
            .add_system(tick_energy.system())
            .add_system(text_update_batteries.system())
            .add_system(update_energy_labels.system())
            .add_system(update_cost_buttons.system());
    }
}

struct World(Timer);
struct ResourceTypeEnergy;
struct BatteryText;

struct MyButton {
    target: Buttons,
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
    disabled: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            disabled: materials.add(Color::rgb(0.45, 0.45, 0.45).into()),
        }
    }
}

fn tick_energy(time: Res<Time>, mut timer: ResMut<World>, mut energy: ResMut<Energy>) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        energy.value += energy.inc_size;
        if energy.value > energy.max {
            energy.value = energy.max;
        }
    }
}

fn update_energy_labels(
    energy: ChangedRes<Energy>,
    mut nrg_text_q: Query<With<ResourceTypeEnergy, &mut Text>>,
) {
    for mut text in &mut nrg_text_q.iter() {
        text.value = format!("{}: {} / {}", energy.name, energy.value, energy.max);
    }
}

fn update_cost_buttons(
    button_materials: Res<ButtonMaterials>,
    costs: Res<Costs>,
    _: ChangedRes<Energy>,
    energy: Res<Energy>,
    mut button_query: Query<(&Button, &mut Handle<ColorMaterial>, &Children)>,
    text_query: Query<(&mut Text, &MyButton)>,
) {
    for (_, mut mat, children) in &mut button_query.iter().iter() {
        if let Ok(btn) = text_query.get_mut::<MyButton>(children[0]) {
            if costs.can_afford(energy.value, &btn.target) {
                *mat = button_materials.normal;
            } else {
                *mat = button_materials.disabled;
            }
        }
    }
}

fn text_update_batteries(
    batteries: ChangedRes<Batteries>,
    mut query: Query<(&mut Text, &BatteryText)>,
) {
    for (mut text, _res) in &mut query.iter() {
        text.value = format!("{}: {}", batteries.name, batteries.value);
    }
}

fn buttons(
    button_materials: Res<ButtonMaterials>,
    costs: Res<Costs>,
    mut batteries: ResMut<Batteries>,
    mut energy: ResMut<Energy>,
    mut iq: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &Children,
    )>,
    text_query: Query<&mut Text>,
) {
    for (_button, interaction, mut material, children) in &mut iq.iter() {
        match *interaction {
            Interaction::Clicked => {
                if !material.eq(&button_materials.disabled) {
                    ()
                }
                *material = button_materials.pressed;
                if let Ok(btn) = text_query.get_mut::<MyButton>(children[0]) {
                    match btn.target {
                        Buttons::BuyBattery => {
                            if costs.can_afford(energy.value, &btn.target) {
                                batteries.value += 1;
                                energy.value -= costs.get(&Buttons::BuyBattery);
                                energy.max = batteries.value * 10;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Interaction::Hovered => {
                if !material.eq(&button_materials.disabled) {
                    *material = button_materials.hovered;
                }
            }
            Interaction::None => {
                if !material.eq(&button_materials.disabled) {
                    *material = button_materials.normal
                }
            }
        }
    }
}

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    let costs = Costs::new();

    let font_handle = asset_server
        .load("assets/fonts/FiraMono-Medium.ttf")
        .unwrap();

    commands
        .spawn(UiCameraComponents::default())
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                padding: Rect::all(Val::Px(15.)),
                ..Default::default()
            },
            // material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
                        ..Default::default()
                    },
                    text: Text {
                        value: "...:".to_string(),
                        font: font_handle,
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    },
                    ..Default::default()
                })
                .with(ResourceTypeEnergy);

            parent
                .spawn(TextComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
                        ..Default::default()
                    },
                    text: Text {
                        value: "batteries:".to_string(),
                        font: font_handle,
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    },
                    ..Default::default()
                })
                .with(BatteryText);

            parent
                .spawn(ButtonComponents {
                    style: Style {
                        size: Size::new(Val::Percent(99.0), Val::Px(30.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextComponents {
                            text: Text {
                                value: format!("{} - Buy battery", costs.get(&Buttons::BuyBattery)),
                                font: font_handle,
                                style: TextStyle {
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            },
                            ..Default::default()
                        })
                        .with(MyButton {
                            target: Buttons::BuyBattery,
                        });
                })
                .with(MyButton {
                    target: Buttons::BuyBattery,
                });

            parent
                .spawn(ButtonComponents {
                    style: Style {
                        size: Size::new(Val::Percent(99.0), Val::Px(30.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextComponents {
                            text: Text {
                                value: format!(
                                    "{} - Hire guy to buy batteries",
                                    costs.get(&Buttons::HireBatteryGuy)
                                ),
                                font: font_handle,
                                style: TextStyle {
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            },
                            ..Default::default()
                        })
                        .with(MyButton {
                            target: Buttons::HireBatteryGuy,
                        });
                })
                .with(MyButton {
                    target: Buttons::HireBatteryGuy,
                });
        });
}
