
use bevy::{prelude::*, input::common_conditions::input_pressed};
use bevy::window::PrimaryWindow;

use super::utils::AABB;
pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(update_sliders.run_if(input_pressed(MouseButton::Left)))
        ;
    }
}

pub fn update_sliders(
    window:      Query<&Window, With<PrimaryWindow>>,
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
            let aabb = AABB::new(&(x, y), &(slider_size.x, slider_size.y));

            if !aabb.has_point(&(pos.x, pos.y)){
                continue; // Mouse not over the slider
            }

            let pcrt: f32;
            match slider.display.layout {
                SliderLayout::Horizontal => {
                    pcrt = ((pos.x - aabb.min_x)/((aabb.max_x - aabb.min_x)*0.98)).clamp(0.0, 1.0);
                }
                SliderLayout::Vertical => {
                    pcrt = ((pos.y - aabb.min_y)/((aabb.max_y- aabb.min_y)*0.98)).clamp(0.0, 1.0);
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

#[derive(Debug, Clone)]
pub struct SliderDisplay {
    pub layout: SliderLayout,
    pub dims:   (f32, f32), // put dims here as they are used for value calculation 
    pub bclr:   Option<[f32; 4]>,
    pub clr:    Option<[f32; 4]>
} 
impl Default for SliderDisplay {
    fn default() -> Self {
        SliderDisplay {
            layout: SliderLayout::Horizontal, 
            dims: (200.0, 20.0),
            clr:  None, 
            bclr: None, 
        }
    }
}




#[derive(Component, Debug, Clone)]
pub struct Slider {
    pub label:  String,
    pub min:    f32,
    pub max:    f32,
    pub value:  f32,
    pub step:   f32,

    // Display settings
    pub display: SliderDisplay
} 

impl Default for Slider {
    fn default() -> Self {
        Slider {label: "".to_string(), min:0.0, max: 1.0, value: 0.5, step: 0.001, display: SliderDisplay{..default()}}
    }
}

impl Slider {

    pub fn spawn(&self, 
                 commands:      &mut Commands, 
                 ass:           &Res<AssetServer>,
                 position_type: PositionType,
                 pos:           &(Val, Val)
                ) -> Entity {

        let mut styles = Slider::styles();
        if let Some(clr) = self.display.clr {
            styles.handle_color = clr;
        }
        if let Some(bclr) = self.display.bclr {
            styles.box_color = bclr;
        }


        let mut box_style = styles.box_style;
        box_style.position_type = position_type;
        box_style.position = UiRect{left: pos.0, top: pos.1, ..default()};
        box_style.size = Size::new(Val::Px(self.display.dims.0), Val::Px(self.display.dims.1));

        let slider_entity = commands.spawn((NodeBundle{
            style: box_style,
            background_color: BackgroundColor(styles.box_color.into()),
            ..default()
          }, 
          SliderBox, 
          self.clone(), 
          Name::new(format!("Slider {}", self.label.clone()))))
          .id();

        let handle_dims = self.get_handle_dims();
        let handle_pos = self.get_handle_pos();

        let mut handle_style = styles.handle_style;
        handle_style.position = UiRect{left: Val::Percent(handle_pos.0), top: Val::Percent(handle_pos.1), ..default()};
        handle_style.size = Size::new(Val::Px(handle_dims.0), Val::Px(handle_dims.1));
      
        let handler_entity = commands.spawn((NodeBundle{
            style: handle_style,
            background_color: BackgroundColor(styles.handle_color.into()),
            ..default()
          }, SliderHandle, Name::new(format!("Slider Handle {}", self.label.clone())))).id();

        let txt_style = TextStyle {font_size: 15.0, color: Color::WHITE,font: ass.load("fonts/lambda.ttf")};
        let label_entity = commands.spawn((TextBundle::from_section(self.get_label(), txt_style), SliderLabel)).id();
        
        commands.entity(slider_entity).push_children(&[handler_entity, label_entity]);

        return slider_entity;

    }

    pub fn styles() -> DefaultSliderStyles {
        DefaultSliderStyles::default()
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
        match self.display.layout {
            SliderLayout::Horizontal => {
                (self.prct()*self.display.dims.0*0.99, self.display.dims.1*0.9)
            }
            SliderLayout::Vertical => {
                (self.display.dims.0*0.9, self.prct()*self.display.dims.1*0.99)
            }
        }
    }
    pub fn get_handle_pos(&self) -> (f32, f32) {
        match self.display.layout {
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





// Default styles for slider box and slider handle 

pub struct DefaultSliderStyles {
    pub box_style: Style,
    pub handle_style: Style,
    pub box_color:      [f32; 4],
    pub handle_color:   [f32; 4]
}

impl Default for DefaultSliderStyles {
    fn default() -> Self {

        Self{
            box_style: Style {
                display:         Display::Flex,
                position_type:   PositionType::Relative,
                direction:       Direction::Inherit,
                flex_direction:  FlexDirection::Row,
                flex_wrap:       FlexWrap::Wrap,
                align_items:     AlignItems::FlexStart,
                align_self:      AlignSelf::Auto,
                align_content:   AlignContent::Stretch,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },

            handle_style: Style {
                display:         Display::Flex,
                position_type:   PositionType::Absolute,
                direction:       Direction::Inherit,
                flex_direction:  FlexDirection::Row,
                flex_wrap:       FlexWrap::Wrap,
                align_items:     AlignItems::FlexStart,
                align_self:      AlignSelf::Auto,
                align_content:   AlignContent::Stretch,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },

            box_color:    [0.7, 0.7, 0.7, 1.0],
            handle_color: [0.9, 0.9, 0.9, 1.0]
    
        }
    }
}
