use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use std::io::{BufWriter, Write};
use std::str::FromStr;
use strsim::levenshtein;
use std::collections::HashSet;
use std::fs::{self, File};

use super::ToggleWireframeEvent;
use super::mtb_ui::{OpenModalEvent, ModalType, AppState};
use mtb_core::planes::{SpawnNewPlaneEvent, EditPlaneEvent, DEFAULT_PLANE_ID};


pub struct MTBConsolePlugin;

impl Plugin for MTBConsolePlugin {
  fn build(&self, app: &mut App) {
      app
      .add_event::<TriggerCommand>()
      .insert_resource(ConsoleInput{text: "".to_string(), back_delay: Timer::from_seconds(0.4, TimerMode::Once)})
      .insert_resource(SentCommands{data: Vec::new(), index:0})
      .insert_resource(PlaneSetID(DEFAULT_PLANE_ID))
      .add_state::<ConsoleState>()
      .add_startup_system(setup)
      .add_systems((console_toggle.run_if(input_just_pressed(KeyCode::Return)), 
                    update.run_if(in_state(ConsoleState::On)), 
                    send_command.run_if(on_event::<TriggerCommand>()), 
                    animate.run_if(in_state(ConsoleState::On)),
                    display.run_if(in_state(ConsoleState::On))).chain())
      ;
  }
}

#[derive(Resource)]
pub struct PlaneSetID(u32);


#[derive(Component)]
struct TextAnimationTimer{
    timer: Timer,
    on: bool
}

#[derive(Component)]
pub struct ConsoleDisplay;

#[derive(Component)]
pub struct ConsoleDisplayText;

#[derive(Resource)]
pub struct ConsoleInput {
    pub back_delay: Timer,
    pub text: String
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ConsoleState {
    #[default]
    Off,
    On
}

#[derive(Resource)]
pub struct SentCommands {
    pub data: Vec<String>,
    pub index: usize
}

impl Drop for SentCommands {
    fn drop(&mut self) {
        // deduplicates commands and cuts the ones that are too old;
        let f = File::create("./assets/data/console.json").ok().unwrap();
        let mut uniq: Vec<String> = self.data.clone().into_iter().collect::<HashSet<String>>().into_iter().collect();

        if uniq.len() > 50 {
            let n_rm = uniq.len() - 50;
            uniq.drain(0..=n_rm);
        }

        let mut writer = BufWriter::new(f);
        let _res = serde_json::to_writer(&mut writer, &uniq);
        let _res = writer.flush();
    }
}


impl SentCommands {
    fn move_index(&mut self, move_by: i32){

        let mut new_index: i32 = self.index as i32 + move_by;

        if new_index >= self.data.len() as i32{
            new_index = self.data.len() as i32 - 1;
        } 
        if new_index <= 0 {
            new_index = 0;
        }
        
        self.index = new_index as usize;
    }

    fn set_last_as_index(&mut self){
        self.index = self.data.len() - 1;
    }
}

pub struct TriggerCommand;


const ALLOWED_CHARS: [&str; 10] = ["_", ".", ",", "-", ":", ";", "|", "/", "\\", " "];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Funcs {
    NewPlaneData,
    PlaneData,
    SetID,
    WireFrame,
    NewColor,
    Editor,
    View
}

impl FromStr for Funcs {
    type Err = ();
    fn from_str(input: &str) -> Result<Funcs, Self::Err> {
        match input {
            "set"   => Ok(Funcs::SetID),
            "npd"   => Ok(Funcs::NewPlaneData),
            "pd"    => Ok(Funcs::PlaneData),
            "wf"    => Ok(Funcs::WireFrame),
            "nc"    => Ok(Funcs::NewColor),
            "e"     => Ok(Funcs::Editor),
            "v"     => Ok(Funcs::View),
            _      => Err(()),
        }
    }
}

fn setup(mut commands: Commands,
         ass: Res<AssetServer>,
         mut sent_commands: ResMut<SentCommands>
        ){

    let console_node = commands.spawn(NodeBundle{
        style: Style {
          position_type: PositionType::Absolute,
          position: UiRect {left: Val::Percent(5.0), 
                            top: Val::Percent(93.0), 
                            ..default()},
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          flex_wrap: FlexWrap::Wrap,
          flex_direction: FlexDirection::Column,
          align_items: AlignItems::FlexStart,
          justify_content: JustifyContent::FlexStart,
          ..default()
        },
        ..default()
      })
      .insert(Name::new("ConsoleText"))
      .insert(ConsoleDisplay)
      .insert(Visibility::Hidden)
      .id();

      let txt_style = TextStyle {
        font: ass.load("fonts/lambda.ttf"),
        font_size: 20.0,
        color: Color::BLACK,
      };

    let txt = commands.spawn(TextBundle::from_sections([TextSection::new("$:> ", txt_style.clone()),
                                                        TextSection::new("",     txt_style.clone()),
                                                        TextSection::new("|",     txt_style.clone())]))
                                                        .insert(ConsoleDisplayText)
                                                        .insert(TextAnimationTimer{timer: Timer::from_seconds(0.4, TimerMode::Repeating), on: true})
                                                        .id();

    commands.entity(console_node).push_children(&[txt]);


    // load sent commands

    let path: &str = "./assets/data/console.json";
    let data: String = read_txt(path);
    sent_commands.data = serde_json::from_str(&data)
                                    .expect(&format!("\n [ERROR console.setup] Unable to get data from {path} \n"));


}

fn console_toggle(console_state:            Res<State<ConsoleState>>,
                  mut next_console_state:   ResMut<NextState<ConsoleState>>,
                  mut trigger_command:      EventWriter<TriggerCommand>,
                  mut console_node:         Query<&mut Visibility, With<ConsoleDisplay>>
                ){

    match console_state.0 {
        ConsoleState::On => {
            trigger_command.send(TriggerCommand);
            for mut v in console_node.iter_mut(){
                *v = Visibility::Hidden;
            }
            next_console_state.set(ConsoleState::Off);
        }

        ConsoleState::Off => {
            for mut v in console_node.iter_mut(){
                *v = Visibility::Inherited;
            }
            next_console_state.set(ConsoleState::On);
        }

    }
    
}
                      
fn update(keys: Res<Input<KeyCode>>,
          time: Res<Time>,
          mut sent_commands: ResMut<SentCommands>,
          mut console: ResMut<ConsoleInput>,
          mut received_char: EventReader<ReceivedCharacter>, ){

    for ev in received_char.iter() {
        if ev.char.is_alphanumeric() ||  ALLOWED_CHARS.contains(&ev.char.to_string().as_str()){
            console.text.push(ev.char);
        }
    }
    if keys.just_pressed(KeyCode::Back){
        console.text.pop();
    }
    if keys.just_pressed(KeyCode::Delete){
        console.text.clear();
    }
    if keys.just_pressed(KeyCode::Up){
        sent_commands.move_index(-1);
        if let Some(cmdtxt) = sent_commands.data.get(sent_commands.index){
            console.text = cmdtxt.clone();
        }
    }
    if keys.just_pressed(KeyCode::Down){
        sent_commands.move_index(1);
        if let Some(cmdtxt) = sent_commands.data.get(sent_commands.index){
            console.text = cmdtxt.clone();
        }
    }
    if keys.just_pressed(KeyCode::Tab){
        let args = get_func_args(&console);
        let last_arg = get_last(&console);
        if args.is_some() && last_arg.is_some(){
            let target = last_arg.unwrap();
            let repl = get_most_similar_alphabetically(&target, &mut args.unwrap());
            let index = console.text.len() - target.len();
            console.text = console.text[..index].to_string() + &repl;
        }
    }

    if keys.pressed(KeyCode::Back){
        console.back_delay.tick(time.delta());
        if console.back_delay.finished(){
            console.text.pop();
        }
    }

    if keys.just_released(KeyCode::Back){
        console.back_delay.reset();
    }

}

fn display(console: Res<ConsoleInput>,
           mut query: Query<&mut Text, With<ConsoleDisplayText>>,){

    if console.is_changed(){
        for mut text in &mut query {
            text.sections[1].value = format!("{}", &console.text);
        }
    }
}

fn animate(mut query: Query<&mut Text, With<ConsoleDisplayText>>,
           time: Res<Time>,
           mut text_timer_query: Query<&mut TextAnimationTimer>
        ){

    for mut timer in text_timer_query.iter_mut(){
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            if timer.on {
                timer.on=false;
                for mut text in &mut query {
                    text.sections[2].value = "".to_string();
                }
            } else {
                timer.on=true;
                for mut text in &mut query {
                    text.sections[2].value = "|".to_string();
                }
            }
        }
    }
}

// Searches for argument in the vector of arg_name:arg_value, returns optional index
fn search_arg_value(arg_name: &str, args: &Vec<&str>) -> Option<String> {
    for arg_str in args.iter(){
        let _arg_name = format!("{}:", arg_name);
        if arg_str.contains(&_arg_name){
            return Some(arg_str.replace(&_arg_name, ""));
        }
    }
    return None;
}


fn get_func_args<'a>(console: &ResMut<ConsoleInput>) -> Option<Vec<&'a str>> {
    let args_split = console.text.split_whitespace();
    let mut args: Vec<&str> = Vec::new();
    for arg in args_split.into_iter(){
        args.push(arg);
    }
    if args.len() >= 1 {
        let func_str = args[0];
        if let Ok(func) = Funcs::from_str(func_str){
            match func {
                Funcs::NewPlaneData             => {return Some(vec!["id", "loc", "dims", "subs"])}
                Funcs::PlaneData                => {return Some(vec!["id", "loc", "dims", "subs", "mod", "clr", "active"])}
                Funcs::SetID                    => {return Some(vec!["id"])}
                Funcs::WireFrame                => {return Some(vec![""])}
                Funcs::NewColor                 => {return Some(vec![""])}
                Funcs::Editor                   => {return Some(vec![""])}
                Funcs::View                     => {return Some(vec![""])}
            }
        }
    } 
    return None;
}

fn get_last(console: &ResMut<ConsoleInput>) -> Option<String>{
    let args_split = console.text.split_whitespace();
    let mut args: Vec<&str> = Vec::new();
    for arg in args_split.into_iter(){
        args.push(arg);
    }
    if args.len() >= 2 {
        if let Some(last_str) = args.last(){
            if !last_str.contains(":"){
                return Some(last_str.to_string());
            } else {
                return None;
            }
        } else {return None;}
    } else {
        return None;
    }
}

#[allow(dead_code)]
fn get_most_similar(target: &str, mut args: Vec<&str>) -> String {
    args.sort_by_key(|arg| levenshtein(arg, target));
    return args[0].to_string();
}

fn get_most_similar_alphabetically(target: &str, args: &mut Vec<&str>) -> String {
    args.sort_by_key(|s| {
        let mut score = 0;
        let mut target_chars = target.chars();
        for c in s.chars() {
            if let Some(tc) = target_chars.next() {
                if c == tc {
                    score += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        score
    });
    return args.last().unwrap().to_string();
}

fn send_command(console:              Res<ConsoleInput>,
                mut trigger_command:  EventReader<TriggerCommand>,
                mut sent_commands:    ResMut<SentCommands>,
                mut spawn_new_plane:  EventWriter<SpawnNewPlaneEvent>,
                mut edit_plane:       EventWriter<EditPlaneEvent>,
                mut toggle_wf:        EventWriter<ToggleWireframeEvent>,
                mut open_modal:       EventWriter<OpenModalEvent>,
                mut plane_set_id:     ResMut<PlaneSetID>,
                mut next_app_state:   ResMut<NextState<AppState>>
            ){

    for _ev in trigger_command.iter() {
        let args_split = console.text.split_whitespace();
        let mut args: Vec<&str> = Vec::new();
        for arg in args_split.into_iter(){
            args.push(arg);
        }
        if args.len() >= 1 {
            let func_str = args[0];
            if let Ok(func) = Funcs::from_str(func_str){
                sent_commands.data.push(console.text.clone());
                sent_commands.set_last_as_index();

                match func {
                    Funcs::NewPlaneData => {
                        let mut snpe = SpawnNewPlaneEvent::new();
                        let opt_id = search_arg_value("id", &args);
                        match (plane_set_id.0, opt_id) {
                            (DEFAULT_PLANE_ID, None) => {info!(" [CONSOLE] Need ID for new plane");  return;}
                            (_, None) => {info!(" [CONSOLE] Setting the plane id from set ({})", plane_set_id.0);  snpe.pd.id = plane_set_id.0;}
                            (_, Some(s)) => {
                                if let Some(id) = unpack_u32(&s, "NewPlaneData", "id") {
                                    snpe.pd.id = id;
                                }
                            }
                        }

                        if let Some(loc_str) = search_arg_value("loc", &args){
                            if let Some(loc) = unpack_array3_f32(&loc_str, "NewPlaneData", "loc") {
                                snpe.pd.loc = loc;
                            }
                        }

                        if let Some(dims_str) = search_arg_value("dims", &args){
                            if let Some(dims) = unpack_array2_f32(&dims_str, "NewPlaneData", "dims") {
                                snpe.pd.dims = dims;
                            }
                        }

                        if let Some(subs_str) = search_arg_value("subs", &args){
                            if let Some(subs) = unpack_array2_u32(&subs_str, "NewPlaneData", "subs") {
                                snpe.pd.subdivisions = subs;
                            }
                        }

                        spawn_new_plane.send(snpe);
                    }
                    Funcs::PlaneData => {

                        let mut epe = EditPlaneEvent::new();
                        let opt_id = search_arg_value("id", &args);
                        match (plane_set_id.0, opt_id) {
                            (DEFAULT_PLANE_ID, None) => {info!(" [CONSOLE] Need ID for new plane");  return;}
                            (_, None) => {info!(" [CONSOLE] Setting the plane id from set ({})", plane_set_id.0);  epe.id = plane_set_id.0;}
                            (_, Some(s)) => {
                                if let Some(id) = unpack_u32(&s, "PlaneData", "id") {
                                    epe.id = id;
                                }
                            }
                        }

                        if let Some(loc_str) = search_arg_value("loc", &args){
                            epe.loc = unpack_array3_f32(&loc_str, "PlaneData", "loc");
                        }
                        if let Some(dims_str) = search_arg_value("dims", &args){
                            epe.dims = unpack_array2_f32(&dims_str, "PlaneData", "dims");
                        }
                        if let Some(subs_str) = search_arg_value("subs", &args){
                            epe.subs = unpack_array2_u32(&subs_str, "PlaneData", "subs");
                        }
                        if let Some(clr_str) = search_arg_value("clr", &args){
                            open_modal.send(OpenModalEvent {modal_type: ModalType::PlaneColor})
                        }

                        info!(" [CONSOLE] Editing plane id {:?}", epe.id);
                        edit_plane.send(epe);
                    }
                    Funcs::SetID => {
                        if let Some(id_str) = search_arg_value("id", &args){
                            if let Some(id) =unpack_u32(&id_str, "SetID", "id"){
                                plane_set_id.0 = id;
                                info!("[CONSOLE] Setting ID to {}", id);
                            } else {
                                info!(" [CONSOLE] Need correct ID (u32) for new plane");
                                return;
                            }
                        }
                    }
                    Funcs::WireFrame => {toggle_wf.send(ToggleWireframeEvent);}
                    Funcs::NewColor => {open_modal.send(OpenModalEvent {modal_type: ModalType::Color})}
                    Funcs::Editor => {next_app_state.set(AppState::Editor)}
                    Funcs::View => {next_app_state.set(AppState::View)}
                    
                }
            } else {
                info!(" [CONSOLE] Invalid function string");
            }
        } else {
            info!(" [CONSOLE] No function string");
            break;
        }
    }

}
#[allow(dead_code)]
fn unpack_u32<'a>(arg_str: &str, func_name: &str, par_name: &str) -> Option<u32>{
    if let Ok(_val) = arg_str.parse::<u32>(){
        return Some(_val);
    } else {
        info!(" [CONSOLE] {} [{}] invalid parameter", func_name, par_name); 
        return None;
    }
}
#[allow(dead_code)]
fn unpack_usize<'a>(arg_str: &str, func_name: &str, par_name: &str) -> Option<usize>{
    if let Ok(_val) = arg_str.parse::<usize>(){
        return Some(_val);
    } else {
        info!(" [CONSOLE] {} [{}] invalid parameter", func_name, par_name); 
        return None;
    }
}

#[allow(dead_code)]
fn unpack_f64<'a>(arg_str: &str, func_name: &str, par_name: &str) -> Option<f64>{
    if let Ok(_val) = arg_str.parse::<f64>(){
        return Some(_val);
    } else {
        info!(" [CONSOLE] {} [{}] invalid parameter", func_name, par_name); 
        return None;
    }
}

fn unpack_array2_u32<'a>(arg_str: &str, func_name: &str, par_name: &str) -> Option<[u32; 2]>{
    let parts = arg_str.split(',');
    let mut arr: [u32; 2] = [0, 0];
    for (index, part) in parts.into_iter().enumerate(){
        let t_part = part.trim();
        let n = t_part.parse::<u32>();
        if n.is_ok() {
            arr[index] = n.ok().unwrap();
        } else {
            info!(" [CONSOLE] {} [{}] invalid parameter", func_name, par_name); 
            return None;
        }
    }
    return Some(arr);
}



fn unpack_array2_f32<'a>(arg_str: &str, func_name: &str, par_name: &str) -> Option<[f32; 2]>{
    let parts = arg_str.split(',');
    let mut arr: [f32; 2] = [0.0, 0.0];
    for (index, part) in parts.into_iter().enumerate(){
        let t_part = part.trim();
        let n = t_part.parse::<f32>();
        if n.is_ok() {
            arr[index] = n.ok().unwrap();
        } else {
            info!(" [CONSOLE] {} [{}] invalid parameter", func_name, par_name); 
            return None;
        }
    }
    return Some(arr);
}


fn unpack_array3_f32<'a>(arg_str: &str, func_name: &str, par_name: &str) -> Option<[f32; 3]>{
    let parts = arg_str.split(',');
    let mut arr: [f32; 3] = [0.0, 0.0, 0.0];
    for (index, part) in parts.into_iter().enumerate(){
        let t_part = part.trim();
        let n = t_part.parse::<f32>();
        if n.is_ok() {
            arr[index] = n.ok().unwrap();
        } else {
            info!(" [CONSOLE] {} [{}] invalid parameter", func_name, par_name); 
            return None;
        }
    }
    return Some(arr);
}

pub fn read_txt(file_path: &str) -> String {
    info!(" [UTILS] Reading text file {file_path}");
    let data: String = fs::read_to_string(file_path)
                          .expect(&format!("\n [ERROR utils.read_txt] Unable to read file {file_path}  \n"));
    return data;
  }
  