use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use mtb_core::planes::{TerrainPlane, Planes, PlanesAsset};
use mtb_core::utils::{MTBConfigData, MTBConfigAsset};


pub struct MTBLoaderPlugin;

impl Plugin for MTBLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(TomlAssetPlugin::<Planes>::new(&["mtbscene.toml"]))
        .add_plugins(TomlAssetPlugin::<MTBConfigData>::new(&["mtbconfig.toml"]))
        .add_systems(Startup, setup_config)
        .add_systems(Update, setup_terrains_file_handle.run_if(on_event::<AssetEvent<MTBConfigData>>()))
        .add_systems(PostUpdate, planes_update.run_if(on_event::<AssetEvent<Planes>>()))
        ;
    }
  }

fn setup_terrains_file_handle(mut commands:         Commands, 
                              ass:                  Res<AssetServer>,
                              mtb_config_assets:    Res<Assets<MTBConfigData>>,
                              mtb_config_handle:    Res<MTBConfigAsset>){

    let scene_file = &mtb_config_assets.get(&mtb_config_handle.0).unwrap().scene_file;
    let path: &str = &format!("mtb_terrain/scenes/{}.mtbscene.toml", scene_file);
    let planes_handle = PlanesAsset(ass.load(path));
    commands.insert_resource(planes_handle);
}


fn setup_config(mut commands:    Commands, 
                ass:             Res<AssetServer>,) {

    let config_handle = MTBConfigAsset(ass.load("mtb_terrain/config.mtbconfig.toml"));
    commands.insert_resource(config_handle);

}

// generates planes
pub fn planes_update(mut commands:           Commands,
                     mut meshes:             ResMut<Assets<Mesh>>,
                     mut materials:          ResMut<Assets<StandardMaterial>>,
                     terrain_planes:         Query<Entity, With<TerrainPlane>>,
                    //  debug_boxes:            Query<Entity, With<DebugModifierBox>>,
                     planes_assets:          Res<Assets<Planes>>,
                     planes_handle:          Res<PlanesAsset>
                    ){

    for entity in terrain_planes.iter(){
        commands.entity(entity).despawn_recursive();
    }
    // for entity in debug_boxes.iter(){
    //     commands.entity(entity).despawn_recursive();
    // }

    for pd in planes_assets.get(&planes_handle.0).unwrap().planes.iter(){
        pd.spawn(&mut commands, &mut meshes, &mut materials);
    }
}
