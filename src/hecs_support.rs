use uuid::Uuid;
use crate::engine::{MissingIDError, PhysicsEngine};
use crate::prelude::{Actor, Solid};

pub struct ActorLink {
    actor_uuid: Uuid
}

pub struct SolidLink {
    solid_uuid: Uuid
}