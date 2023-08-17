
use bevy::prelude::*;
use super::slider::{Slider, SliderDisplay};
pub struct ColorPickerPlugin;

impl Plugin for ColorPickerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, update_color_picker)
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
        position_type:      PositionType::Absolute,
        left:               Val::Percent(10.0), 
        top:                Val::Percent(10.0), 
        width:              Val::Percent(80.0), 
        height:             Val::Percent(80.0),
        flex_wrap:          FlexWrap::Wrap,
        flex_direction:     FlexDirection::Row,
        align_items:        AlignItems::FlexStart,
        justify_content:    JustifyContent::FlexStart,
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
    let red_slider = Slider{min: 0.0, max: 1.0, value: 0.5, step: 0.01, label: "Red".to_string(),
                        display: SliderDisplay{dims: (200.0, 30.0), clr: Some([0.9, 0.2, 0.2, 0.5]), ..default()}, ..default()};
    let r_ent = red_slider.spawn(commands, &ass, PositionType::Absolute, &(Val::Percent(1.0), Val::Percent(10.0)));

    let green_slider = Slider{min: 0.0, max: 1.0, value: 0.5, step: 0.01, label: "Green".to_string(),
                        display: SliderDisplay{dims: (200.0, 30.0), clr: Some([0.2, 0.9, 0.2, 0.5]), ..default()}, ..default()};
    let g_ent = green_slider.spawn(commands, &ass, PositionType::Absolute, &(Val::Percent(1.0), Val::Percent(20.0)));

    let blue_slider = Slider{min: 0.0, max: 1.0, value: 0.5, step: 0.01, label: "Blue".to_string(),
                        display: SliderDisplay{dims: (200.0, 30.0), clr: Some([0.2, 0.2, 0.9, 0.5]), ..default()}, ..default()};
    let b_ent = blue_slider.spawn(commands, &ass, PositionType::Absolute, &(Val::Percent(1.0), Val::Percent(30.0)));

    let alpha_slider = Slider{min: 0.0, max: 1.0, value: 0.5, step: 0.01, label: "Alpha".to_string(),
                        display: SliderDisplay{dims: (200.0, 30.0), clr: Some([0.984, 0.905, 0.937, 0.5]), ..default()}, ..default()};
    let a_ent = alpha_slider.spawn(commands, &ass, PositionType::Absolute, &(Val::Percent(1.0), Val::Percent(40.0)));

    commands.entity(r_ent).insert(ColorPickerChannel::R);
    commands.entity(g_ent).insert(ColorPickerChannel::G);
    commands.entity(b_ent).insert(ColorPickerChannel::B);
    commands.entity(a_ent).insert(ColorPickerChannel::A);

    // display square
    let display_ent = commands.spawn((NodeBundle{
        style: Style {
          position_type:    PositionType::Absolute,
          left:             Val::Percent(50.0), 
          top:              Val::Percent(10.0), 
          width:            Val::Px(200.0), 
          height:           Val::Px(200.0),
          flex_wrap:        FlexWrap::Wrap,
          flex_direction:   FlexDirection::Row,
          align_items:      AlignItems::FlexStart,
          justify_content:  JustifyContent::FlexStart,
          ..default()
        },
        background_color: BackgroundColor([0.5, 0.5, 0.5, 1.0].into()),
        ..default()
      }, ColorPickerDisplay)).id();

    v.append(&mut vec![r_ent, g_ent, b_ent, a_ent, display_ent]);
    commands.entity(ent).push_children(&v);
    return ent;

  }