use bevy_egui::{egui, egui::Ui};
use bevy::prelude::ResMut;
use serde::{Deserialize,Serialize};

use crate::editor::mtb_ui::ModResources;

use super::easings::Easings;
use super::utils::{Axis, get_distance_euclidean};
use std::slice::Iter;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ValueType {
    Value,
    Diff,
    Scale
}
impl ValueType {
    pub fn apply(&self, value: f32, pos: &[f32;3]) -> f32 {
        match &self {
            ValueType::Value => {return value}
            ValueType::Diff  => {return pos[1] + value;}
            ValueType::Scale => {return pos[1]*value;}
        }
    }
}

impl<'a> ValueType {
    pub fn iterator() -> Iter<'static, ValueType> {
        static OPTIONS: [ValueType; 3] = [
            ValueType::Value,
            ValueType::Diff,
            ValueType::Scale
        ];
        OPTIONS.iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ValueScaling {
    None,
    DistancePoint,
    DistancePointRev,
    DistanceAxis,
    DistanceAxisRev
}

impl<'a> ValueScaling {
    pub fn iterator() -> Iter<'static, ValueScaling> {
        static OPTIONS: [ValueScaling; 5] = [
            ValueScaling::None,
            ValueScaling::DistancePoint,
            ValueScaling::DistancePointRev,
            ValueScaling::DistanceAxis,
            ValueScaling::DistanceAxisRev
        ];
        OPTIONS.iter()
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Value {
    pub _value:     f32,
    pub _scale:     f32,
    pub _axis:      Axis,
    pub _axis_v:    f32,
    pub _point:     (f32, f32),
    pub value_type: ValueType,
    pub easing:     Easings,
    pub scaling:    ValueScaling,
}

impl Value {    

    pub fn new() -> Self {
        Value{_value:     10.0, 
              _scale:     1.0,
              _axis:      Axis::X,
              _axis_v:    0.0,
              _point:     (0.0, 0.0),
              value_type: ValueType::Value, 
              easing:     Easings::None, 
              scaling:    ValueScaling::None}
    }

    pub fn apply(&self, pos: &[f32;3]) -> f32 {
        let v2: f32 = self.value_type.apply(self._value, pos);

        match self.scaling {
            ValueScaling::DistanceAxisRev => {
                let dist: f32;
                match self._axis {
                    Axis::X => {dist = get_distance_euclidean(&(pos[0], pos[2]), &(self._axis_v, pos[2]));}
                    Axis::Z => {dist = get_distance_euclidean(&(pos[0], pos[2]), &(pos[0], self._axis_v));}
                }
                let mut scale: f32 = 1.0 - 1.0/dist;
                scale = self.easing.apply(scale);
                return v2*scale;
            }
            ValueScaling::DistanceAxis => {
                let dist: f32;
                match self._axis {
                    Axis::X => {dist = get_distance_euclidean(&(pos[0], pos[2]), &(self._axis_v, pos[2]));}
                    Axis::Z => {dist = get_distance_euclidean(&(pos[0], pos[2]), &(pos[0], self._axis_v));}
                }
                let mut scale: f32 = 1.0/dist;
                scale = self.easing.apply(scale);
                return v2*scale;
            }
            ValueScaling::DistancePointRev => {
                let dist = get_distance_euclidean(&(pos[0], pos[2]), &self._point);
                let mut scale: f32 = 1.0 - 1.0/dist;
                scale = self.easing.apply(scale);
                return v2*scale;
            }
            ValueScaling::DistancePoint => {
                let dist = get_distance_euclidean(&(pos[0], pos[2]), &self._point);
                let mut scale: f32 = 1.0/dist;
                scale = self.easing.apply(scale);
                return v2*scale;
            }
            ValueScaling::None => {return v2;}
        }
    }   

    pub fn ui(ui: &mut Ui, mod_res: &mut ResMut<ModResources>) {
        
        ui.label("Value");
        ui.add(egui::DragValue::new(&mut mod_res.value._value).speed(0.1));
        ui.vertical(|ui| {
          ui.label("Type:");
          for &p in ValueType::iterator(){
              if ui.radio_value(&mut mod_res.value.value_type, p, format!("{p:?}")).clicked() {
                mod_res.value.value_type = p;
              };
          }
        });
        ui.separator();
        ui.vertical(|ui| {
          ui.label("Scaling:");
          for &p in ValueScaling::iterator(){
              if ui.radio_value(&mut mod_res.value.scaling, p, format!("{p:?}")).clicked() {
                mod_res.value.scaling = p;
              };
          }
        });

        match mod_res.value.scaling {
          ValueScaling::DistanceAxis | ValueScaling::DistanceAxisRev => {
            ui.vertical(|ui| {
              ui.label("Axis:");
              for &p in Axis::iterator(){
                  if ui.radio_value(&mut mod_res.value._axis, p, format!("{p:?}")).clicked() {
                    mod_res.value._axis = p;
                  };
              }
              ui.label("Axis Value:");
              ui.add(egui::DragValue::new(&mut mod_res.value._axis_v).speed(1.0));
              ui.label("Scale:");
              ui.add(egui::DragValue::new(&mut mod_res.value._scale).speed(0.1));
            });
          }
          ValueScaling::DistancePoint | ValueScaling::DistancePointRev => {
            ui.vertical(|ui| {
              ui.label("Point:");
              ui.columns(2, |columns| {
                columns[1].label("X");
                columns[0].add(egui::DragValue::new(&mut mod_res.value._point.0).speed(1.0));
                columns[1].label("Y");
                columns[0].add(egui::DragValue::new(&mut mod_res.value._point.1).speed(1.0));
              });
              ui.label("Scale:");
              ui.add(egui::DragValue::new(&mut mod_res.value._scale).speed(0.1));
            });
          }
          _ => {}
        }

        ui.separator();

    }

}