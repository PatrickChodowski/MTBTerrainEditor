use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel, MouseMotion};
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::window::PrimaryWindow;
use libm::atan2f; 

const CENTER_X: f32 = 0.0;
const CENTER_Z: f32 = 0.0;
const CAMERA_START_Y: f32 = 200.0;
const CAMERA_START_Z: f32 = 200.0; 
const CAMERA_SPEED: f32 = 600.0;
const CAMERA_SENSITIVITY: f32 = 0.0001; 

pub struct MTBCameraPlugin;

impl Plugin for MTBCameraPlugin {
  fn build(&self, app: &mut App) {
      app
      .init_resource::<InputState>()
      .add_startup_system(setup)
      .add_system(zoom_camera)
      .add_system(move_camera)
      .add_system(pan_look)
      .add_system(set_camera)
      ;
  }
}

pub fn get_yaw(q: Quat) -> f32 {
  return atan2f(2.0*q.y*q.w - 2.0*q.x *q.z, 1.0 - 2.0*q.y*q.y - 2.0*q.z*q.z);
}

pub fn get_pitch(q: Quat) -> f32 {
  return atan2f(2.0*q.x*q.w - 2.0*q.y*q.z, 1.0 - 2.0*q.x*q.x - 2.0*q.z*q.z);
}

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

#[derive(Component)]
pub struct MTBCamera;


fn setup(mut commands: Commands, 
         mut state: ResMut<InputState>,){

  let start_camera_transform = Transform::from_xyz(CENTER_X, CAMERA_START_Y, CAMERA_START_Z)
                                         .looking_at([CENTER_X, 0.0, CENTER_Z].into(), Vec3::Y);
  commands.spawn((Camera3dBundle {
                  transform: start_camera_transform,
                  ..default()}, MTBCamera));

  state.yaw = get_yaw(start_camera_transform.rotation);
  state.pitch = get_pitch(start_camera_transform.rotation);
  state.pitch = state.pitch.clamp(-1.54, 1.54);

}


fn move_camera(keys:         Res<Input<KeyCode>>,
               mut query:    Query<&mut Transform, With<MTBCamera>>,
               time:         Res<Time>){

  let mut transform = query.single_mut();
  let mut velocity = Vec3::ZERO;
  let local_z = transform.local_z();
  let forward = -Vec3::new(local_z.x, 0., local_z.z);
  let right = Vec3::new(local_z.z, 0., -local_z.x);

  for key in keys.get_pressed() {
    match key {
      KeyCode::W => velocity += forward,
      KeyCode::S => velocity -= forward,
      KeyCode::A => velocity -= right,
      KeyCode::D => velocity += right,
      KeyCode::E => velocity += Vec3::Y,
      KeyCode::Q => velocity -= Vec3::Y,
      _ => (),
    }
  } 
  
  velocity = velocity.normalize_or_zero();
  transform.translation += velocity * CAMERA_SPEED * time.delta_seconds();

}

fn zoom_camera(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<MTBCamera>>){

  for mouse_wheel_event in mouse_wheel_events.iter() {
    let dy = match mouse_wheel_event.unit {
      MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
      MouseScrollUnit::Pixel => mouse_wheel_event.y,
    };

    for mut transform in query.iter_mut(){
      transform.translation.y -= dy;
    }
  }
}

fn pan_look(windows: Query<&Window, With<PrimaryWindow>>,
            motion: Res<Events<MouseMotion>>,
            buttons: Res<Input<MouseButton>>,
            mut state: ResMut<InputState>,
            mut query: Query<&mut Transform, With<MTBCamera>>,){

  if buttons.pressed(MouseButton::Middle) {
    if let Ok(window) = windows.get_single() {        
      let delta_state = state.as_mut();
      for mut transform in query.iter_mut() {
        for ev in delta_state.reader_motion.iter(&motion) {
          let window_scale = window.height().min(window.width());
          delta_state.pitch -= (CAMERA_SENSITIVITY * ev.delta.y * window_scale).to_radians();
          delta_state.yaw -= (CAMERA_SENSITIVITY * ev.delta.x * window_scale).to_radians();
        }
        delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);
        transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)* Quat::from_axis_angle(Vec3::X, delta_state.pitch);
      }
    }
  }
}



fn set_camera(keys: Res<Input<KeyCode>>, 
              mut query: Query<&mut Transform, With<MTBCamera>>,
              mut state: ResMut<InputState>,
            ){

  if keys.any_just_pressed([KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5]){
    let pressed_key = keys.get_just_pressed().next().unwrap();
    let xyz: (f32, f32, f32);
    match pressed_key {
      KeyCode::Key1 => {xyz = (CENTER_X, CAMERA_START_Y, CAMERA_START_Z)}
      _ => {xyz = (0.0, 0.0, 0.0)}
    }
    for mut transform in query.iter_mut() {
      *transform = Transform::from_xyz(xyz.0, xyz.1, xyz.2).looking_at([CENTER_X, 0.0, CENTER_Z].into(), Vec3::Y);
      state.yaw = get_yaw(transform.rotation);
      state.pitch = get_pitch(transform.rotation);
      state.pitch = state.pitch.clamp(-1.54, 1.54);
    }
  }
}
