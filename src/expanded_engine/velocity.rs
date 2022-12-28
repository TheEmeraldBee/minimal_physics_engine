use macroquad::math::Vec2;
use macroquad::prelude::get_frame_time;
use uuid::Uuid;
use crate::actor::CollisionCallback;
use crate::engine::{MissingIDError, PhysicsEngine};

pub struct ActorVelocity {
    pub uuid: Uuid,
    pub velocity: Vec2
}

impl ActorVelocity {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid,
            velocity: Vec2::splat(0.0)
        }
    }

    pub fn update(&mut self, engine: &mut PhysicsEngine) -> Result<(), MissingIDError>{
        let my_actor = engine.actor_storage.get_actor(self.uuid)?;
        if self.velocity.x != 0.0 || self.velocity.y != 0.0 {
            my_actor.move_actor(self.velocity * get_frame_time(), CollisionCallback::None, &engine.solid_storage.solids);
        }

        Ok(())
    }
}

pub struct SolidVelocity {
    pub uuid: Uuid,
    pub velocity: Vec2
}

impl SolidVelocity {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid,
            velocity: Vec2::splat(0.0)
        }
    }

    pub fn update(&mut self, engine: &mut PhysicsEngine) -> Result<(), String>{
        if self.velocity.x != 0.0 || self.velocity.y != 0.0 {
            engine.move_solid(self.uuid, self.velocity * get_frame_time())?;
        }

        Ok(())
    }
}