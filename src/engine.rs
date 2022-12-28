use std::error::Error;
use std::fmt::{Display, Formatter};
use macroquad::math::Vec2;
use uuid::Uuid;
use crate::actor::CollisionCallback;
use crate::prelude::{Actor, Collider, Solid};
use crate::solid::SolidInteraction;

pub struct PhysicsEngine {
    pub actor_storage: ActorStorage,
    pub solid_storage: SolidStorage
}

pub struct ActorStorage {
    pub actors: Vec<Actor>
}

impl ActorStorage {
    pub fn get_actor(&mut self, actor_uuid: Uuid) -> Result<&mut Actor, MissingIDError> {
        let mut my_actor_option = None;
        for actor in self.actors.iter_mut() {
            if actor.uuid == actor_uuid {
                my_actor_option = Some(actor)
            }
        }

        return if let Some(my_actor) = my_actor_option {
            Ok(my_actor)
        } else {
            Err(MissingIDError { error: format!("Expected an actor with uuid {}, but did not find one.", actor_uuid) })
        }
    }
}

pub struct SolidStorage {
    pub solids: Vec<Solid>
}

impl SolidStorage {
    pub fn get_solid(&mut self, solid_uuid: Uuid) -> Result<&mut Solid, MissingIDError> {
        let mut my_solid_option = None;
        for solid in self.solids.iter_mut() {
            if solid.uuid == solid_uuid {
                my_solid_option = Some(solid)
            }
        }

        return if let Some(my_solid) = my_solid_option {
            Ok(my_solid)
        } else {
            Err(MissingIDError { error: format!("Expected an solid with id {}, but did not find one.", solid_uuid) })
        }
    }

    pub fn get_solids_with_tag(&mut self, tag: &str) -> Vec<Uuid> {
        let mut solids = vec![];

        for solid in self.solids.iter_mut() {
            if solid.has_tag(tag) {
                solids.push(solid.uuid);
            }
        }

        solids
    }
}

#[derive(Debug)]
pub struct MissingIDError {
    pub error: String
}

impl Display for MissingIDError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for MissingIDError {}

impl PhysicsEngine {
    pub fn new() -> Self {
        Self {
            actor_storage: ActorStorage {actors: vec![]},
            solid_storage: SolidStorage {solids: vec![]}
        }
    }

    /// ignore_id is the id of the solid that we need to ignore collisions with.
    pub fn handle_interactions(&mut self, interactions: &Vec<SolidInteraction>, ignore_uuid: Uuid) {

        // Retain the solid that was the interaction has.
        let mut solids = self.solid_storage.solids.clone();
        solids.retain(|x| x.uuid != ignore_uuid);

        for interaction in interactions {
            for actor in self.actor_storage.actors.iter_mut() {
                if actor.uuid == interaction.actor_uuid {
                    actor.move_exact(interaction.motion, CollisionCallback::Squish, &solids)
                }
            }
        }
    }

    pub fn move_solid(&mut self, solid_uuid: Uuid, distance: Vec2) -> Result<(), String> {
        for solid_index in 0..self.solid_storage.solids.len() {
            if self.solid_storage.solids[solid_index].uuid == solid_uuid {
                let y_interactions = self.solid_storage.solids[solid_index].move_y(distance.y, &mut self.actor_storage.actors);
                self.handle_interactions(&y_interactions, self.solid_storage.solids[solid_index].uuid);

                let x_interactions = self.solid_storage.solids[solid_index].move_x(distance.x, &mut self.actor_storage.actors);
                self.handle_interactions(&x_interactions, self.solid_storage.solids[solid_index].uuid);

                return Ok(())
            }
        }

        Err(format!("No Actor with UUID: {}.", solid_uuid))
    }

    /// Must be run at end of function to clean up other calls.
    pub fn update(&mut self) {
        for actor in self.actor_storage.actors.iter_mut() {
            actor.update();
        }
    }

    /// Spawns a solid and returns the id of the solid
    pub fn spawn_solid(&mut self, solid_collider: Collider, tags: Option<Vec<String>>) -> Uuid {
        let mut uuid;

        loop {
            uuid = Uuid::new_v4();
            match self.solid_storage.get_solid(uuid) {
                Ok(_) => { }
                Err(_) => { break; }
            }
        }

        self.solid_storage.solids.push(Solid::new(solid_collider, uuid, tags));

        uuid
    }

    /// Spawns an actor and returns the id of the actor
    pub fn spawn_actor(&mut self, actor_collider: Collider) -> Uuid {
        let mut uuid;

        loop {
            uuid = Uuid::new_v4();
            match self.actor_storage.get_actor(uuid) {
                Ok(_) => { }
                Err(_) => { break; }
            }
        }

        self.actor_storage.actors.push(Actor::new(actor_collider, uuid));

        uuid
    }
}