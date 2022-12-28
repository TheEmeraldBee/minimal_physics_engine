use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::actor::CollisionCallback;
use crate::prelude::{Actor, Solid};
use crate::solid::SolidInteraction;

pub struct PhysicsEngine {
    pub actor_storage: ActorStorage,
    pub solid_storage: SolidStorage
}

pub struct ActorStorage {
    pub actors: Vec<Actor>
}

impl ActorStorage {
    pub fn get_actor(&mut self, actor_id: i32) -> Result<&mut Actor, MissingIDError> {
        // Try to get the actor in the engine
        let mut my_actor_option = None;
        for actor in self.actors.iter_mut() {
            if actor.id == actor_id {
                my_actor_option = Some(actor)
            }
        }

        return if let Some(my_actor) = my_actor_option {
            Ok(my_actor)
        } else {
            Err(MissingIDError { error: format!("Expected an actor with id {}, but did not find one.", actor_id) })
        }
    }
}

pub struct SolidStorage {
    pub solids: Vec<Solid>
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
    pub fn handle_interactions(&mut self, interactions: &Vec<SolidInteraction>, ignore_id: i32) {

        // Retain the solid that was the interaction has.
        let mut solids = self.solid_storage.solids.clone();
        solids.retain(|x| x.id != ignore_id);

        for interaction in interactions {
            for actor in self.actor_storage.actors.iter_mut() {
                if actor.id == interaction.actor_id {
                    actor.move_exact(interaction.motion, CollisionCallback::Squish, &solids)
                }
            }
        }
    }

    /// Must be run at end of function to clean up other calls.
    pub fn update(&mut self) {
        for actor in self.actor_storage.actors.iter_mut() {
            actor.update();
        }
    }
}