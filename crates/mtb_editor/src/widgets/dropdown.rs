
use bevy::{prelude::*, input::common_conditions::input_just_pressed};
use bevy::window::PrimaryWindow;

use super::utils::AABB;
pub struct DropDownPlugin;

impl Plugin for DropDownPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(update_dropdown.run_if(input_just_pressed(MouseButton::Left)))
        ;
    }
}

#[derive(Component)]
pub struct DropDownLabel;

#[derive(Component, Clone, Debug)]
pub struct DropDownOption {
    pub index: u32,
    pub label: String,
    pub value: String
}


#[derive(Component, Clone, Debug)]
pub struct DropDown {
    pub label:          String,
    pub options:        Vec<DropDownOption>,
    pub expanded:       bool,

    pub display:        DropDownDisplay
}

impl Default for DropDown {
    fn default() -> Self {
        DropDown {label: "Default Dropdown".to_string(), 
                  options: vec![
                    DropDownOption{index: 0, label: "Option 1".to_string(), value: "option1".to_string()},
                    DropDownOption{index: 1, label: "Option 2".to_string(), value: "option2".to_string()},
                    DropDownOption{index: 2, label: "Option 3".to_string(), value: "option3".to_string()}
                  ],
                  expanded: false,
                  display: DropDownDisplay::default()
                }
    }
}


impl DropDown {

    pub fn spawn(&self,                  
                 commands:      &mut Commands, 
                 ass:           &Res<AssetServer>,
                 position_type: PositionType,
                 pos:           &(Val, Val)) -> Entity {

        let mut styles = DropDown::styles();
        if let Some(label_color) = self.display.label_color {
            styles.label_color = label_color;
        }
        if let Some(option_color) = self.display.option_color {
            styles.option_color = option_color;
        }


        let mut label_style = styles.label_style;
        label_style.position_type = position_type;
        label_style.position = UiRect{left: pos.0, top: pos.1, ..default()};
        label_style.size = Size::new(Val::Px(self.display.dims.0), Val::Px(self.display.dims.1));

        let dropdown_entity = commands.spawn((NodeBundle{
                style: label_style,
                background_color: BackgroundColor(styles.label_color.into()),
                ..default()
            }, 
            DropDownLabel, 
            self.clone(), 
            Name::new(format!("Dropdown {}", self.label.clone()))))
            .id();

        let mut v: Vec<Entity> = Vec::new();
        let txt_style = TextStyle {font_size: 15.0, color: Color::BLACK,font: ass.load("fonts/lambda.ttf")};
        let label_entity = commands.spawn(TextBundle::from_section(self.label.clone(), txt_style.clone())).id();
        v.push(label_entity);

        let mut option_style = styles.option_style;
        option_style.position_type = PositionType::Relative;
        // option_style.position = UiRect{left: Val::Px(5.0), top: Val::Px(25.0), ..default()};
        option_style.position = UiRect{left: Val::Percent(0.0), top: Val::Percent(70.0), ..default()};
        option_style.margin = UiRect{left: Val::Px(0.0), top: Val::Px(5.0), right: Val::Px(0.0), bottom: Val::Px(0.0)};
        option_style.size = Size::new(Val::Px(self.display.option_dims.0), Val::Px(self.display.option_dims.1));

        for ddopt in self.options.iter(){

            let dropdown_option_entity = commands.spawn((NodeBundle{
                style: option_style.clone(),
                background_color: BackgroundColor(styles.option_color.into()),
                ..default()
            }, 
            ddopt.clone(), 
            Name::new(format!("Dropdown Option {}", ddopt.label.clone()))))
            .id();

            let label_option_entity = commands.spawn(TextBundle::from_section(ddopt.label.clone(), txt_style.clone())).id();
            commands.entity(dropdown_option_entity).push_children(&[label_option_entity]);

            v.push(dropdown_option_entity);
        }

        commands.entity(dropdown_entity).push_children(&v);

        return dropdown_entity;

    }

    pub fn styles() -> DefaultDropDownStyles {
        DefaultDropDownStyles::default()
    }

}



#[derive(Debug, Clone)]
pub struct DropDownDisplay {
    pub dims:           (f32, f32), 
    pub option_dims:    (f32, f32),
    pub label_color:    Option<[f32; 4]>,
    pub option_color:   Option<[f32; 4]>
} 
impl Default for DropDownDisplay {
    fn default() -> Self {
        DropDownDisplay {
            dims: (110.0, 40.0),
            option_dims: (100.0, 40.0),
            label_color:  None, 
            option_color: None, 
        }
    }
}



pub struct DefaultDropDownStyles {
    pub label_style:    Style,
    pub option_style:   Style,
    pub label_color:    [f32; 4],
    pub option_color:   [f32; 4]
}

impl Default for DefaultDropDownStyles {
    fn default() -> Self {

        Self{
            label_style: Style {
                display:         Display::Flex,
                position_type:   PositionType::Relative,
                direction:       Direction::Inherit,
                flex_direction:  FlexDirection::Column,
                flex_wrap:       FlexWrap::NoWrap,
                align_items:     AlignItems::Center,
                align_self:      AlignSelf::Auto,
                align_content:   AlignContent::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },

            option_style: Style {
                display:         Display::None,
                position_type:   PositionType::Absolute,
                direction:       Direction::Inherit,
                flex_direction:  FlexDirection::Column,
                flex_wrap:       FlexWrap::Wrap,
                align_items:     AlignItems::Center,
                align_self:      AlignSelf::Auto,
                align_content:   AlignContent::Stretch,
                justify_content: JustifyContent::Center,
                ..default()
            },

            label_color:  [0.7, 0.7, 0.7, 1.0],
            option_color: [0.9, 0.9, 0.9, 1.0]
    
        }
    }
}



fn update_dropdown(window:          Query<&Window, With<PrimaryWindow>>,
                   mut dropdowns:   Query<(&Node, &Visibility, &GlobalTransform, &Children, &mut DropDown), Without<DropDownOption>>,
                   mut dd_options:  Query<&mut Style, With<DropDownOption>>)
{

    let Ok(primary) = window.get_single() else {return;};
    if let Some(pos) = primary.cursor_position(){
        for (n, v, gt, children, mut dropdown) in dropdowns.iter_mut(){
            if v == Visibility::Hidden {
                continue;
            }
            
            let x = gt.translation().x;
            let y = primary.height() - gt.translation().y;
            let dd_size = n.size();
            let aabb = AABB::new(&(x, y), &(dd_size.x, dd_size.y));

            if !aabb.has_point(&(pos.x, pos.y)){
                continue; // Mouse not over the slider
            }

            dropdown.expanded = !dropdown.expanded;
            for child in children.iter(){
                if let Ok(mut style) = dd_options.get_mut(*child) {
                    if dropdown.expanded {
                        style.display = Display::Flex;
                    } else {
                        style.display = Display::None;
                    }
                }
            }
        }
    }
}
