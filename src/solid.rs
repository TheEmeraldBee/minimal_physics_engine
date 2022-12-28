use macroquad::math::Vec2;
use uuid::Uuid;
use crate::actor::{Actor};
use crate::math::{vec2i32, Vec2I32};
use crate::prelude::Collider;

#[derive(Clone, Debug, PartialEq)]
pub struct Solid {
    pub uuid: Uuid,
    pub remainder: Vec2,
    pub collider: Collider
}

impl Solid {

    /// Requires an ID. Id MUST be different than other SOLID's ID's.
    pub fn new(collider: Collider, uuid: Uuid) -> Self {
        Self {
            uuid,
            remainder: Default::default(),
            collider
        }
    }

    pub fn move_x(&mut self, distance: f32, actors: &mut Vec<Actor>) -> Vec<SolidInteraction> {
        self.remainder.x += distance;
        let move_distance = self.remainder.x as i32;
        self.remainder.x -= move_distance as f32;

        let mut interactions = vec![];

        if move_distance != 0 {
            self.collider.x += move_distance;

            for actor in actors.iter_mut() {
                if self.collider.is_overlapping(vec2i32(0, 0), &actor.collider) {
                    if move_distance > 0 {
                        interactions.push(SolidInteraction {
                            actor_uuid: actor.uuid,
                            motion: vec2i32(self.collider.max_x() - actor.collider.min_x(), 0)
                        });
                    } else {
                        interactions.push(SolidInteraction {
                            actor_uuid: actor.uuid,
                            motion: vec2i32(self.collider.min_x() - actor.collider.max_x(), 0)
                        });
                    }
                } else if actor.is_riding(self) {
                    interactions.push(SolidInteraction {
                        actor_uuid: actor.uuid,
                        motion: vec2i32(move_distance, 0)
                    })
                }
            }
        }

        interactions
    }

    pub fn move_y(&mut self, distance: f32, actors: &mut Vec<Actor>) -> Vec<SolidInteraction> {
        self.remainder.y += distance;
        let move_distance = self.remainder.y as i32;
        self.remainder.y -= move_distance as f32;

        let mut interactions = vec![];

        if move_distance != 0 {
            self.collider.y += move_distance;

            for actor in actors.iter_mut() {
                if self.collider.is_overlapping(vec2i32(0, 0), &actor.collider) {
                    if move_distance > 0 {
                        interactions.push(SolidInteraction {
                            actor_uuid: actor.uuid,
                            motion: vec2i32(0, self.collider.max_y() - actor.collider.min_y())
                        });
                    }
                    else {
                        interactions.push(SolidInteraction {
                            actor_uuid: actor.uuid,
                            motion: vec2i32(0, self.collider.min_y() - actor.collider.max_y())
                        });
                    }

                }
                else if actor.is_riding(self) {
                    interactions.push(SolidInteraction {
                        actor_uuid: actor.uuid,
                        motion: vec2i32(0, move_distance)
                    })
                }
            }
        }

        interactions
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SolidInteraction {
    pub actor_uuid: Uuid,
    pub motion: Vec2I32
}