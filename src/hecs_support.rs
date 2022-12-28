use crate::engine::{MissingIDError, PhysicsEngine};
use crate::prelude::{Actor, Solid};

pub struct ActorLink {
    actor_id: i32
}

impl ActorLink {
    pub fn get_actor(&mut self, engine: &mut PhysicsEngine) -> Result<&mut Actor, MissingIDError> {
        engine.actor_storage.get_actor(self.actor_id)
    }
}

pub struct SolidLink {
    solid_id: i32
}

impl SolidLink {
    pub fn get_solid(&mut self, engine: &mut PhysicsEngine) -> Result<&mut Solid, MissingIDError> {
        engine.solid_storage.get_solid(self.solid_id)
    }
}