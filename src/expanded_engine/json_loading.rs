use std::str::FromStr;
use macroquad::file::{load_string};
use crate::engine::PhysicsEngine;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::prelude::Collider;
use crate::solid::Solid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonLevel {
    level_solids: Vec<JsonCollider>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonCollider {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    tags: Option<Vec<String>>,
    uuid: Option<String>
}

/// Loads a json level format to the engine via a path.
pub async fn load_level(engine: &mut PhysicsEngine, path: &str, clear_solids: bool) -> Result<(), String> {
    if clear_solids {
        engine.solid_storage.solids.clear();
    }

    let file = match load_string(path).await {
        Ok(file) => { file },
        Err(_) => { return Err(format!("Could not load file with name {path}")); }
    };

    let level = match serde_json::from_str::<JsonLevel>(file.as_str()) {
        Ok(level) => { level }
        Err(_) => { return Err(format!("File {path} did not have the correct formatting to become a level.")) }
    };

    for collider in level.level_solids {
        match collider.uuid {
            Some(uuid) => { engine.solid_storage.solids.push(Solid::new(Collider::new(collider.x, collider.y, collider.width, collider.height), match Uuid::from_str(uuid.as_str()) {
                Ok(uuid) => uuid,
                Err(error) => {return Err(format!("{}", error))}
            }, collider.tags)) }
            None => { engine.spawn_solid(Collider::new(collider.x, collider.y, collider.width, collider.height), collider.tags); }
        }

    }

    Ok(())
}

/// Takes all spawned solids and prints it as a jsonified level.
pub fn save_level(engine: &mut PhysicsEngine, save_uuid: bool) -> Result<(), String> {
    let mut level = JsonLevel {
        level_solids: vec![]
    };

    for solid in engine.solid_storage.solids.iter() {
        level.level_solids.push( JsonCollider {x: solid.collider.x, y: solid.collider.y, width: solid.collider.width, height: solid.collider.height, tags: None,
            uuid: match save_uuid { true => {Some(solid.uuid.hyphenated().to_string()) }, false => { None } }}
        );
    }

    let level_json = match serde_json::to_string_pretty(&level) {
        Ok(level_json) => { level_json }
        // We SHOULD Never Get Here
        Err(_) => { return Err("Error, could not read level as json.".to_string()) }
    };

    println!("{}", level_json);

    Ok(())

}