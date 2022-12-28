use macroquad::math::Vec2;
use macroquad::time::get_frame_time;
use crate::prelude::*;

pub struct ActorVelocity {
    pub id: i32,
    pub velocity: Vec2
}

impl ActorVelocity {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            velocity: Vec2::splat(0.0)
        }
    }

    pub fn update(&mut self, engine: &mut PhysicsEngine) -> Result<(), MissingIDError>{
        let mut my_actor = engine.actor_storage.get_actor(self.id)?;
        if self.velocity.x != 0.0 || self.velocity.y != 0.0 {
            my_actor.move_actor(self.velocity * get_frame_time(), CollisionCallback::None, &engine.solid_storage.solids);
        }

        Ok(())
    }
}

pub struct SolidVelocity {
    pub id: i32,
    pub velocity: Vec2
}

impl SolidVelocity {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            velocity: Vec2::splat(0.0)
        }
    }

    pub fn update(&mut self, engine: &mut PhysicsEngine) -> Result<(), MissingIDError>{
        if self.velocity.x != 0.0 || self.velocity.y != 0.0 {
            engine.move_solid(self.id, self.velocity * get_frame_time());
        }

        Ok(())
    }
}