
use std::error::Error;
use bevy::input::common_conditions::input_pressed;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use crate::core::utils::get_distance_euclidean;
use std::fmt::{Display, Formatter};
use bevy::prelude::Mesh;
use bevy::math::{Rect, Vec2, Vec3};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use triangulate::{ListFormat, Vertex as TRIVertex, TriangulationError};
use triangulate::formats::IndexedListFormat;

use crate::core::vertex::{Vertex, PickedVertex};
use super::mtb_ui::PickerState;
use super::mtb_grid::HoverData;

pub struct BrushPlugin;

impl Plugin for BrushPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(BrushSettings::new())
        .add_systems(OnExit(PickerState::Brush), despawn_brush)
        .add_systems(OnEnter(PickerState::Brush), spawn_brush)
        .add_systems(Update, update_brush.run_if(in_state(PickerState::Brush)))
        .add_systems(Update, select.after(update_brush).run_if(input_pressed(MouseButton::Left).and_then(in_state(PickerState::Brush))))
        ;
    }
}

fn select(brush_settings:    Res<BrushSettings>,
          brush_select:      Query<(&Transform, &Brush)>,
          mut vertex:        Query<(&GlobalTransform, &mut PickedVertex), With<Vertex>>
){
    if let Ok((brt, _br)) = brush_select.get_single(){
        for (gtr, mut picked) in vertex.iter_mut() {
            // get translation
            let tr = gtr.translation();
            if get_distance_euclidean(&(brt.translation.x, brt.translation.z), &(tr.x, tr.z)) <= brush_settings.radius {
                picked.0 = true;
            } 
        }
    }
}

#[derive(Resource)]
pub struct BrushSettings {
  pub radius: f32
}
impl BrushSettings {
    fn new() -> Self {
        BrushSettings {radius: 20.0}
    }
}

#[derive(Component)]
pub struct Brush;

#[derive(Component)]
pub struct BrushSettingRadius;


pub fn update_brush(hover_data:        Res<HoverData>,
                    brush_settings:    Res<BrushSettings>,
                    mut brush:         Query<&mut Transform, With<Brush>>){

    if let Ok(mut t) = brush.get_single_mut(){
        t.translation = [hover_data.hovered_xz.0, 20.0, hover_data.hovered_xz.1].into();

        if brush_settings.is_changed() {
            let scale = brush_settings.radius;
            t.scale = [scale, 1.0, scale].into();
        }

    }
}

pub fn spawn_brush(mut commands:      Commands, 
                   mut materials:     ResMut<Assets<StandardMaterial>>,
                   mut meshes:        ResMut<Assets<Mesh>>, 
                   brush_settings:    Res<BrushSettings>,
                   brush:             Query<&Transform, With<Brush>>,
                   hover_data:        Res<HoverData>){

    let loc = hover_data.hovered_xz;
    if brush.is_empty(){
        let scale = brush_settings.radius;
        commands.spawn((PbrBundle {
            mesh: meshes.add(Mesh::try_from(Polygon::new_regular_ngon(1.0, 32)).unwrap()),
            material: materials.add(StandardMaterial::from(Color::YELLOW.with_a(0.4))),
            transform: Transform::from_xyz(loc.0, 20.0, loc.1).with_scale([scale, 1.0, scale].into()),
            ..default()
        }, Brush, NotShadowCaster));
    }
}

// Deselects all selectable on mouse right click or button change
pub fn despawn_brush(mut commands:     Commands,
                     brush:            Query<Entity, With<Brush>>){ 
  for ent in brush.iter(){
    commands.entity(ent).despawn();
  }
}



// From bevy more shapes:
// credit: https://github.com/redpandamonium/bevy_more_shapes/


pub struct Polygon {
    pub points: Vec<Vec2>,
}

impl Polygon {
    pub fn new_regular_ngon(radius: f32, n: usize) -> Polygon {
        let angle_step = 2.0 * std::f32::consts::PI / n as f32;
        let mut points = Vec::with_capacity(n);

        for i in 0..n {
            let theta = angle_step * i as f32;
            points.push(Vec2::new(
                radius * f32::cos(theta),
                radius * f32::sin(theta),
            ));
        }

        Polygon { points }
    }
}

fn bounding_rect_for_points<'a>(points: impl Iterator<Item = &'a Vec2>) -> Rect {
    let mut x_min = 0.0f32;
    let mut x_max = 0.0f32;
    let mut y_min = 0.0f32;
    let mut y_max = 0.0f32;

    for point in points {
        x_min = x_min.min(point.x);
        x_max = x_max.max(point.x);
        y_min = y_min.min(point.y);
        y_max = y_max.max(point.y);
    }

    Rect {
        min: Vec2::new(x_min, y_min),
        max: Vec2::new(x_max, y_max),
    }
}

#[derive(Debug)]
pub struct InvalidInput;

impl Display for InvalidInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid polygon input")
    }
}

impl Error for InvalidInput { }

impl<T: Error> From<TriangulationError<T>> for InvalidInput {
    fn from(value: TriangulationError<T>) -> Self {
        match value {
            TriangulationError::TrapezoidationError(_) => panic!("Failed to triangulate: {}", value),
            TriangulationError::NoVertices => Self,
            TriangulationError::InternalError(_) => Self,
            TriangulationError::FanBuilder(_) => panic!("Failed to triangulate: {}", value),
            _ => panic!("Failed to triangulate: {}", value),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Vec2f(Vec2);
impl TRIVertex for Vec2f {
    type Coordinate = f32;

    fn x(&self) -> Self::Coordinate {
        self.0.x
    }

    fn y(&self) -> Self::Coordinate {
        self.0.y
    }
}



impl TryFrom<Polygon> for Mesh {

    type Error = InvalidInput;

    fn try_from(polygon: Polygon) -> Result<Self, Self::Error> {

        if polygon.points.len() < 3 {
            return Err(InvalidInput);
        }

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(polygon.points.len());
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(polygon.points.len());
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(polygon.points.len());

        // The domain is needed for UV mapping. The domain tells us how to transform all points to optimally fit the 0-1 range.
        let domain = bounding_rect_for_points(polygon.points.iter());

        // Add the vertices
        for v in &polygon.points {
            positions.push([v.x, 0.0, v.y]);
            normals.push(Vec3::Y.to_array());

            // Transform the polygon domain to the 0-1 UV domain.
            let u = (v.x - domain.min.x) / (domain.max.x - domain.min.x);
            let v = (v.y - domain.min.y) / (domain.max.y - domain.min.y);
            uvs.push([u, v]);
        }

        // Triangulate to obtain the indices
        // This library is terrible to use. The heck is that initializer object. And this trait madness.
        let polygons = polygon
            .points
            .into_iter()
            .map(|v| Vec2f(v))
            .collect::<Vec<Vec2f>>();
        let mut output = Vec::<[usize; 3]>::new();
        let format = IndexedListFormat::new(&mut output).into_fan_format();
        triangulate::Polygon::triangulate(&polygons, format)?;
        let indices = output.into_iter()
            .map(|[a, b, c]| [c, b, a])
            .flatten()
            .map(|v| v as u32)
            .collect();

        // Put the mesh together
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(indices)));
        Ok(mesh)
    }
}
