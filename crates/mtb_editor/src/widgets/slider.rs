
use bevy::{prelude::*, input::common_conditions::input_pressed};
use bevy::window::PrimaryWindow;

pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(update_sliders.run_if(input_pressed(MouseButton::Left)))
        ;
    }
}

pub fn update_sliders(
    window:  Query<&Window, With<PrimaryWindow>>,
    mut sliders: Query<(&Node, &Visibility, &GlobalTransform, &Children, &mut Slider), (Without<SliderHandle>, Without<SliderLabel>)>,
    mut handles: Query<&mut Style, (With<SliderHandle>, Without<SliderLabel>)>,
    mut labels:  Query<&mut Text, (With<SliderLabel>, Without<SliderHandle>)>
){

    let Ok(primary) = window.get_single() else {return;};
    if let Some(pos) = primary.cursor_position(){
        for (n, v, gt, children, mut slider) in sliders.iter_mut(){
            if v == Visibility::Hidden {
                continue;
            }
            
            let x = gt.translation().x;
            let y = primary.height() - gt.translation().y;
            let slider_size = n.size();
            let aabb = get_aabb(&(x, y), &(slider_size.x, slider_size.y));

            if !(pos.x >= aabb[0] && pos.x <= aabb[1] && pos.y >= aabb[2] && pos.y <= aabb[3]){
                continue; // Mouse not over the slider
            }

            let pcrt: f32;
            match slider.layout {
                SliderLayout::Horizontal => {
                    pcrt = ((pos.x - aabb[0])/((aabb[1]- aabb[0])*0.98)).clamp(0.0, 1.0);
                }
                SliderLayout::Vertical => {
                    pcrt = ((pos.y - aabb[2])/((aabb[3]- aabb[2])*0.98)).clamp(0.0, 1.0);
                }
            }
            slider.map(pcrt);
            for child_ent in children.iter() {
                if let Ok(mut style) = handles.get_mut(*child_ent) {
                    let handle_dims = slider.get_handle_dims();
                    style.size.width = Val::Px(handle_dims.0);
                    style.size.height = Val::Px(handle_dims.1);
                }

                if let Ok(mut text) = labels.get_mut(*child_ent) {
                    text.sections[0].value = slider.get_label();
                }
            }
            
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SliderLayout {
    Vertical,
    Horizontal
}

#[derive(Component, Debug, Clone)]
pub struct Slider {
    pub label:  String,
    pub min:    f32,
    pub max:    f32,
    pub value:  f32,
    pub step:   f32,

    pub layout: SliderLayout,
    pub dims:   (f32, f32)
} 

impl Slider {
    pub fn new(label: String) -> Self {
        Slider {label, min:0.0, max: 1.0, value: 0.5, step: 0.001, layout: SliderLayout::Horizontal, dims: (200.0, 20.0)}
    }

    pub fn from_dims(label: String, dims: (f32, f32)) -> Self {
        Slider {label, min:0.0, max: 1.0, value: 0.5, step: 0.001, layout: SliderLayout::Horizontal, dims}
    }

    pub fn map(&mut self, pcrt: f32) {
        let range = self.max - self.min;
        self.value = range*pcrt + self.min;
        let inv = 1.0/self.step;

        if self.value >= range/2.0 {
            self.value = (f32::ceil(self.value * inv)/inv).clamp(self.min, self.max);
        } else {
            self.value = (f32::floor(self.value * inv)/inv).clamp(self.min, self.max);
        }
    }

    pub fn prct(&self) -> f32 {
        return ((self.value + self.min)/((self.max - self.min))).clamp(0.0, 1.0); 
    }

    pub fn get_handle_dims(&self) -> (f32, f32) {
        match self.layout {
            SliderLayout::Horizontal => {
                (self.prct()*self.dims.0*0.99, self.dims.1*0.9)
            }
            SliderLayout::Vertical => {
                (self.dims.0*0.9, self.prct()*self.dims.1*0.99)
            }
        }
    }
    pub fn get_handle_pos(&self) -> (f32, f32) {
        match self.layout {
            SliderLayout::Horizontal => {
                (0.5, 5.0)
            }
            SliderLayout::Vertical => {
                (5.0, 0.5)
            }
        }
    }

    pub fn get_label(&self) -> String {
        format!("{}: {:.3}", self.label, self.value)
    }
}


#[derive(Component)]
pub struct SliderBox;

#[derive(Component)]
pub struct SliderHandle;

#[derive(Component)]
pub struct SliderLabel;

pub fn spawn_slider(commands: &mut Commands, ass: &Res<AssetServer>, xy: &(f32, f32), dims: &(f32, f32), clr: [f32; 4], label: String) -> Entity {

    let slider = Slider::from_dims(label.clone(), *dims);
    let ent_slider = commands.spawn((NodeBundle{
        style: Style {
          position_type: PositionType::Absolute,
          position: UiRect {left: Val::Percent(xy.0), 
                            top: Val::Percent(xy.1), 
                            ..default()},
          size: Size::new(Val::Px(slider.dims.0), Val::Px(slider.dims.1)),
          flex_wrap: FlexWrap::Wrap,
          flex_direction: FlexDirection::Row,
          align_items: AlignItems::FlexStart,
          justify_content: JustifyContent::FlexStart,
          ..default()
        },
        background_color: BackgroundColor([0.7, 0.7, 0.7, 1.0].into()),
        ..default()
      }, SliderBox, slider.clone(), Name::new(format!("Slider {}", label.clone())))).id();

    let handle_dims = slider.get_handle_dims();
    let handle_pos = slider.get_handle_pos();

    let ent_handler = commands.spawn((NodeBundle{
        style: Style {
          position_type: PositionType::Absolute,
          position: UiRect {left: Val::Percent(handle_pos.0), 
                            top: Val::Percent(handle_pos.1), 
                            ..default()},
          size: Size::new(Val::Px(handle_dims.0), Val::Px(handle_dims.1)),
          flex_wrap: FlexWrap::Wrap,
          flex_direction: FlexDirection::Row,
          align_items: AlignItems::FlexStart,
          justify_content: JustifyContent::FlexStart,
          ..default()
        },
        background_color: BackgroundColor(clr.into()),
        ..default()
      }, SliderHandle, Name::new(format!("Slider Handle {}", label.clone())))).id();

    // label:
    let txt_style = TextStyle {font_size: 15.0, color: Color::WHITE,font: ass.load("fonts/lambda.ttf")};
    let label_ent = commands.spawn((TextBundle::from_section(slider.get_label(), txt_style), SliderLabel)).id();
    
    commands.entity(ent_slider).push_children(&[ent_handler, label_ent]);
    return ent_slider;
}

// minimal node aabb [min_x, max_x, min_y, max_y]
fn get_aabb(xy: &(f32, f32), dims: &(f32, f32)) -> [f32; 4] {
    [xy.0 - dims.0/2.0, xy.0 + dims.0/2.0, xy.1 - dims.1/2.0, xy.1 + dims.1/2.0]
}