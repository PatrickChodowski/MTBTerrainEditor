
use bevy::prelude::*;
use super::slider::{spawn_slider, SliderPlugin, Slider};
pub struct ColorPickerPlugin;

impl Plugin for ColorPickerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(SliderPlugin)
        .add_system(update_color_picker)
        ;
    }
}


#[derive(Component, Debug)]
pub struct ColorPickerData {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}
impl ColorPickerData {
    pub fn new() -> Self {
        ColorPickerData {r: 0.0, g: 0.0, b: 0.0, a: 1.0}
    }
}

#[derive(Component)]
pub enum ColorPickerChannel {
    R,G,B,A
}

#[derive(Component)]
pub struct ColorPickerDisplay;



pub fn update_color_picker(mut color_picker:       Query<&mut ColorPickerData, Without<ColorPickerDisplay>>,
                           colors:                 Query<(&Slider, &ColorPickerChannel)>,
                           mut color_display:      Query<&mut BackgroundColor, (With<ColorPickerDisplay>, Without<ColorPickerData>)>
){
    if let Ok(mut cpd) = color_picker.get_single_mut() {
        for (slider, cpc) in colors.iter(){
            match cpc {
                ColorPickerChannel::R => {cpd.r = slider.value }
                ColorPickerChannel::G => {cpd.g = slider.value }
                ColorPickerChannel::B => {cpd.b = slider.value }
                ColorPickerChannel::A => {cpd.a = slider.value }
            }
        }

        if let Ok(mut bkgc) = color_display.get_single_mut() {
            bkgc.0.set_r(cpd.r);
            bkgc.0.set_g(cpd.g);
            bkgc.0.set_b(cpd.b);
            bkgc.0.set_a(cpd.a);
        }
    }
}


pub fn spawn_color_picker(commands: &mut Commands, ass: &Res<AssetServer>) -> Entity {

    let ent = commands.spawn(NodeBundle{
      style: Style {
        position_type: PositionType::Absolute,
        position: UiRect {left: Val::Percent(10.0), 
                          top: Val::Percent(10.0), 
                          ..default()},
        size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
        flex_wrap: FlexWrap::Wrap,
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart,
        ..default()
      },
      background_color: BackgroundColor([0.6, 0.6, 0.6, 1.0].into()),
      ..default()
    })
    .insert(ColorPickerData::new())
    .insert(Name::new("Color Picker"))
    .id()
    ;

    // Color sliders:
    let mut v: Vec<Entity> = Vec::new();
    let r_ent = spawn_slider(commands, ass, &(1.0, 10.0), &(200.0, 30.0), [0.9, 0.2, 0.2, 0.5], "Red".to_string());
    let g_ent = spawn_slider(commands, ass, &(1.0, 20.0), &(200.0, 30.0), [0.2, 0.9, 0.2, 0.5], "Green".to_string());
    let b_ent = spawn_slider(commands, ass, &(1.0, 30.0), &(200.0, 30.0), [0.2, 0.2, 0.9, 0.5], "Blue".to_string());
    let a_ent = spawn_slider(commands, ass, &(1.0, 40.0), &(200.0, 30.0), [0.984, 0.905, 0.937, 0.5], "Alpha".to_string());
    commands.entity(r_ent).insert(ColorPickerChannel::R);
    commands.entity(g_ent).insert(ColorPickerChannel::G);
    commands.entity(b_ent).insert(ColorPickerChannel::B);
    commands.entity(a_ent).insert(ColorPickerChannel::A);

    // display square
    let display_ent = commands.spawn((NodeBundle{
        style: Style {
          position_type: PositionType::Absolute,
          position: UiRect {left: Val::Percent(50.0), 
                            top: Val::Percent(10.0), 
                            ..default()},
          size: Size::new(Val::Px(200.0), Val::Px(200.0)),
          flex_wrap: FlexWrap::Wrap,
          flex_direction: FlexDirection::Row,
          align_items: AlignItems::FlexStart,
          justify_content: JustifyContent::FlexStart,
          ..default()
        },
        background_color: BackgroundColor([0.5, 0.5, 0.5, 1.0].into()),
        ..default()
      }, ColorPickerDisplay)).id();

    v.append(&mut vec![r_ent, g_ent, b_ent, a_ent, display_ent]);
    commands.entity(ent).push_children(&v);
    return ent;

  }