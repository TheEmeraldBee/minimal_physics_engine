use macroquad::math::Vec2;
use uuid::Uuid;
use crate::actor::CollisionCallback;
use crate::math::{Vec2I32};
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
    pub fn get_actor(&mut self, actor_uuid: Uuid) -> Result<&mut Actor, String> {
        let mut my_actor_option = None;
        for actor in self.actors.iter_mut() {
            if actor.uuid == actor_uuid {
                my_actor_option = Some(actor)
            }
        }

        return if let Some(my_actor) = my_actor_option {
            Ok(my_actor)
        } else {
            Err(format!("Expected an actor with uuid {}, but did not find one.", actor_uuid))
        }
    }
}

pub struct SolidStorage {
    pub solids: Vec<Solid>
}

impl SolidStorage {
    pub fn get_solid(&mut self, solid_uuid: Uuid) -> Result<&mut Solid, String> {
        let mut my_solid_option = None;
        for solid in self.solids.iter_mut() {
            if solid.uuid == solid_uuid {
                my_solid_option = Some(solid)
            }
        }

        return if let Some(my_solid) = my_solid_option {
            Ok(my_solid)
        } else {
            Err(format!("Expected an solid with id {}, but did not find one.", solid_uuid))
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

impl PhysicsEngine {
    pub fn new() -> Self {
        Self {
            actor_storage: ActorStorage {actors: vec![]},
            solid_storage: SolidStorage {solids: vec![]}
        }
    }

    fn handle_interactions(&mut self, interactions: &Vec<SolidInteraction>, ignore_uuid: Uuid) {

        // Retain the solid that was the interaction has.
        let mut solids = self.solid_storage.solids.clone();
        solids.retain(|x| x.uuid != ignore_uuid);

        for interaction in interactions {
            for actor in self.actor_storage.actors.iter_mut() {
                if actor.uuid == interaction.actor_uuid {
                    actor.last_push_amount.x += interaction.motion.x;
                    actor.last_push_amount.y += interaction.motion.y;
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

        Err(format!("No Solid with UUID: {}", solid_uuid))
    }

    pub fn move_actor(&mut self, actor_uuid: Uuid, distance: Vec2) -> Result<(), String> {
        for actor in self.actor_storage.actors.iter_mut() {
            if actor.uuid == actor_uuid {
                actor.move_actor(distance, CollisionCallback::None, &self.solid_storage.solids);
                return Ok(())
            }
        }

        Err(format!("No Actor with UUID: {}", actor_uuid))
    }

    pub fn check_overlapping_solid(&mut self, actor_uuid: Uuid, check_offset: Vec2I32) -> Result<bool, String> {
        let actor = self.actor_storage.get_actor(actor_uuid)?;

        for solid in self.solid_storage.solids.iter() {
            if actor.collider.is_overlapping(check_offset, &solid.collider) {
                return Ok(true)
            }
        }

        Ok(false)
    }

    pub fn get_overlapping_solids(&mut self, actor_uuid: Uuid, check_offset: Vec2I32) -> Result<Vec<Uuid>, String> {
        let actor = self.actor_storage.get_actor(actor_uuid)?;
        let mut colliding_uuids = vec![];

        for solid in self.solid_storage.solids.iter() {
            if actor.collider.is_overlapping(check_offset, &solid.collider) {
                colliding_uuids.push(solid.uuid);
            }
        }

        Ok(colliding_uuids)
    }

    pub fn ride(&mut self, actor_uuid: Uuid, solid_uuid: Uuid) -> Result<(), String> {
        let actor = self.actor_storage.get_actor(actor_uuid)?;
        actor.ride(solid_uuid);
        Ok(())
    }

    pub fn check_squished(&mut self, actor_uuid: Uuid) -> Result<bool, String> {
        Ok(self.actor_storage.get_actor(actor_uuid)?.squished)
    }

    /// Must be run at end of function to clean up the engine.
    pub fn end_update(&mut self) {
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