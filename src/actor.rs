use macroquad::prelude::Vec2;
use crate::collider::Collider;
use crate::math::{vec2i32, Vec2I32};
use crate::solid::Solid;

#[derive(Copy, Clone)]
pub enum CollisionCallback {
    None,
    Squish
}

pub trait Riding {
    fn is_riding(&self, actor: &Actor, solid: &Solid) -> bool;
}

pub struct Actor {
    pub id: i32,
    pub remainder: Vec2,
    pub collider: Collider,
    pub squished: bool,
    riding: Vec<i32>,
}

impl Actor {
    /// Make sure id is unused.
    pub fn new(collider: Collider, id: i32) -> Self {
        Self {
            id,
            remainder: Default::default(),
            collider,
            squished: false,
            riding: vec![]
        }
    }

    pub fn move_actor(&mut self, distance: Vec2, callback: CollisionCallback, solids: &Vec<Solid>) {
        self.move_x(distance.x, callback, solids);
        self.move_y(distance.y, callback, solids);
    }

    pub fn move_x(&mut self, distance: f32, callback: CollisionCallback, solids: &Vec<Solid>) {
        self.remainder.x += distance;
        let move_amount = self.remainder.x as i32;
        self.remainder.x -= move_amount as f32;

        self.move_x_exact(move_amount, callback, solids);
    }

    pub fn move_y(&mut self, distance: f32, callback: CollisionCallback, solids: &Vec<Solid>) {
        self.remainder.y += distance;
        let move_amount = self.remainder.y as i32;
        self.remainder.y -= move_amount as f32;

        self.move_y_exact(move_amount, callback, solids);
    }

    pub fn move_exact(&mut self, distance: Vec2I32, callback: CollisionCallback, solids: &Vec<Solid>) {
        self.move_x_exact(distance.x, callback, solids);
        self.move_y_exact(distance.y, callback, solids);
    }

    pub fn move_x_exact(&mut self, distance: i32, callback: CollisionCallback, solids: &Vec<Solid>) {
        let mut move_amount = distance;
        let step = move_amount.signum();
        while move_amount != 0 {

            for solid in solids {
                // Check for collision
                if self.collider.is_overlapping(vec2i32(step, 0), &solid.collider) {
                    self.handle_callback(callback);
                    return;
                }
            }

            self.collider.x += step;
            move_amount -= step;
        }
    }

    pub fn move_y_exact(&mut self, distance: i32, callback: CollisionCallback, solids: &Vec<Solid>) {
        let mut move_amount = distance;
        let step = move_amount.signum();
        while move_amount != 0 {
            for solid in solids {
                // Check for collision
                if self.collider.is_overlapping(vec2i32(0, step), &solid.collider) {
                    self.handle_callback(callback);
                    return;
                }
            }

            self.collider.y += step;
            move_amount -= step;
        }
    }

    fn handle_callback(&mut self, callback: CollisionCallback) {
        match callback {
            CollisionCallback::None => { }
            CollisionCallback::Squish => {
                self.squished = true;
            }
        }
    }

    pub(crate) fn update(&mut self) {
        self.riding.clear();
    }

    pub fn ride(&mut self, solid: &Solid) {
        self.riding.push(solid.id);
    }

    pub fn is_riding(&self, solid: &Solid) -> bool {
        self.riding.contains(&solid.id)
    }

    pub fn is_touching_solid(&self, offset: Vec2I32, solids: &Vec<Solid>) -> bool {
        for solid in solids.iter() {
            if self.collider.is_overlapping(offset, &solid.collider) {
                return true;
            }
        }

        false
    }
}