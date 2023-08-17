

use bevy::prelude::*;
use bevy::input::common_conditions::input_just_pressed;
use bevy::window::PrimaryWindow;

use super::utils::AABB;

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<FocusTextState>()
        .add_systems(PreUpdate, try_focus.run_if(input_just_pressed(MouseButton::Left)))
        .add_systems(Update, update_text_input.after(try_focus).run_if(in_state(FocusTextState::On)))
        ;
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum FocusTextState {
    #[default]
    Off,
    On
}

// fitting system name
pub fn try_focus(mut commands:           Commands,
                 window:                 Query<&Window, With<PrimaryWindow>>,
                 mut textboxes:          Query<(Entity, &mut BackgroundColor, &Node, &Visibility, &GlobalTransform), (With<TextInputBox>, Without<FocusedTextInputBox>)>,
                 focused:                Query<Entity, With<FocusedTextInputBox>>,
                 mut next_focus_state:   ResMut<NextState<FocusTextState>>,
                ){

    for entity in focused.iter(){
        commands.entity(entity).remove::<FocusedTextInputBox>();
    }
    next_focus_state.set(FocusTextState::Off);

    let Ok(primary) = window.get_single() else {return;};
    if let Some(pos) = primary.cursor_position(){
        for (entity, mut bkgc, n, v, gt) in textboxes.iter_mut(){
            if v == Visibility::Hidden {
                continue;
            }
            let x = gt.translation().x;
            let y = primary.height() - gt.translation().y;
            let slider_size = n.size();
            let aabb = AABB::new(&(x, y), &(slider_size.x, slider_size.y));
            if aabb.has_point(&(pos.x, pos.y)) {
                bkgc.0.set_r(0.9);
                bkgc.0.set_g(0.9);
                bkgc.0.set_b(0.9);
                bkgc.0.set_a(1.0);
                commands.entity(entity).insert(FocusedTextInputBox);
                next_focus_state.set(FocusTextState::On);
            } else {
                bkgc.0.set_r(0.4);
                bkgc.0.set_g(0.4);
                bkgc.0.set_b(0.4);
                bkgc.0.set_a(1.0);
                continue;
            }
        }
    }

}


const ALLOWED_CHARS: [&str; 10] = ["_", ".", ",", "-", ":", ";", "|", "/", "\\", " "];
pub fn update_text_input(keys: Res<Input<KeyCode>>,
                         time: Res<Time>,
                         mut received_char:    EventReader<ReceivedCharacter>, 
                         mut focused:          Query<(&Children, &mut TextInputBox), With<FocusedTextInputBox>>,
                         mut focused_text:     Query<(&mut Text, &mut TextInput), Without<FocusedTextInputBox>>){

                            
    for (children, mut textbox) in focused.iter_mut(){
        for child in children.iter(){
            if let Ok((mut text, mut text_input)) = focused_text.get_mut(*child){
                for ev in received_char.iter() {
                    if ev.char.is_alphanumeric() ||  ALLOWED_CHARS.contains(&ev.char.to_string().as_str()){
                        text_input.text.push(ev.char);
                    }
                }
                if keys.just_pressed(KeyCode::Back){
                    text_input.text.pop();
                }
                if keys.just_pressed(KeyCode::Delete){
                    text_input.text.clear();
                }
                if keys.pressed(KeyCode::Back){
                    text_input.back_delay.tick(time.delta());
                    if text_input.back_delay.finished(){
                        text_input.text.pop();
                    }
                }
            
                if keys.just_released(KeyCode::Back){
                    text_input.back_delay.reset();
                }
                text.sections[0].value = format!("{}", &text_input.text);
                textbox.id = text_input.id.clone();
                textbox.text = text_input.text.clone();
            }
        }
    }
}




pub fn spawn_text_input(commands: &mut Commands, 
                        ass: &Res<AssetServer>, 
                        xy: &(f32, f32), 
                        dims: &(f32, f32),
                        id: String
                    ) -> Entity {

    let ent_textinput = commands.spawn((NodeBundle{
        style: Style {
          position_type: PositionType::Absolute,
          left: Val::Percent(xy.0), 
          top: Val::Percent(xy.1), 
          width: Val::Px(dims.0), 
          height: Val::Px(dims.1),
          flex_wrap: FlexWrap::Wrap,
          flex_direction: FlexDirection::Row,
          align_items: AlignItems::FlexStart,
          justify_content: JustifyContent::FlexStart,
          ..default()
        },
        background_color: BackgroundColor([0.4, 0.4, 0.4, 1.0].into()),
        ..default()
      }, TextInputBox::new(), Name::new(format!("TextInputBox")))).id();
      
      let txt_style = TextStyle {font_size: 15.0, color: Color::BLACK,font: ass.load("fonts/lambda.ttf")};
      let actual_text = commands.spawn((TextBundle::from_section("", txt_style), TextInput::new(id))).id();
      
      commands.entity(ent_textinput).push_children(&[actual_text]);
      return ent_textinput;
}

#[derive(Component, Debug, Clone)]
pub struct TextInput {
    pub id:   String,
    pub text: String,
    pub back_delay: Timer
}

impl TextInput {
    pub fn new(id: String) -> Self {
        return TextInput{id,
                         text: "".to_string(), 
                         back_delay: Timer::from_seconds(0.4, TimerMode::Once)};
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct FocusedTextInputBox;

#[derive(Component, Debug, Clone)]
pub struct TextInputBox{
    pub id:   String,
    pub text: String
}

impl TextInputBox {
    pub fn new() -> Self {
        return TextInputBox { id:"".to_string(), text: "".to_string()};
    }
}
