extern crate minimal_physics_engine;
extern crate macroquad;
extern crate core;

use std::error::Error;
use macroquad::prelude::*;
use minimal_physics_engine::actor::Actor;
use minimal_physics_engine::engine::PhysicsEngine;
use minimal_physics_engine::prelude::*;

pub struct Player {
    actor_id: i32,
    velocity: Vec2
}

impl Player {
    pub fn handle_riding(&mut self, engine: &mut PhysicsEngine) -> Result<(), MissingIDError> {
        let my_actor = engine.actor_storage.get_actor(self.actor_id)?;

        for solid in &engine.solid_storage.solids {
            if my_actor.collider.is_overlapping(vec2i32(0, -1), &solid.collider) {
                my_actor.ride(&solid);
            }
        }

        Ok(())
    }

    pub fn update(&mut self, engine: &mut PhysicsEngine) -> Result<(), MissingIDError> {
        let my_actor = engine.actor_storage.get_actor(self.actor_id)?;

        let mut motion = 0;
        if is_key_down(KeyCode::Right) {
            motion += 1;
        }
        if is_key_down(KeyCode::Left) {
            motion -= 1;
        }

        self.velocity.x = motion as f32 * 150.0;

        self.velocity.y -= 150.0 * get_frame_time();

        if my_actor.is_touching_solid(vec2i32(0, -1), &engine.solid_storage.solids) && is_key_down(KeyCode::Space) {
            // Jump!
            self.velocity.y = 200.0;
        }

        my_actor.move_actor(self.velocity * get_frame_time(), CollisionCallback::None, &engine.solid_storage.solids);

        Ok(())
    }
}

#[macroquad::main("Physics Engine Example")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = PhysicsEngine::new();

    let mut player = Player { actor_id: 0, velocity: vec2(0.0, 0.0) };

    engine.actor_storage.actors.push(Actor::new(Collider::new(0, 0, 25, 50), 0));

    engine.solid_storage.solids.push(Solid::new(Collider::new(-50, -125, 250, 25), 0));

    'running: loop {
        set_camera(&Camera2D {
            zoom: vec2(1.0 / screen_width(), 1.0/screen_height()),
            ..Default::default()
        });

        player.handle_riding(&mut engine)?;

        player.update(&mut engine)?;

        for actor in engine.actor_storage.actors.iter_mut() {
            draw_rectangle(actor.collider.x as f32, actor.collider.y as f32, actor.collider.width as f32, actor.collider.height as f32, Color::new(0.5, 0.5, 0.5, 1.0));
        }

        engine.move_solid(0, Vec2::new(20.0 * get_frame_time(), 20.0 * get_frame_time()));

        for solid in engine.solid_storage.solids.iter() {
            draw_rectangle(solid.collider.x as f32, solid.collider.y as f32, solid.collider.width as f32, solid.collider.height as f32, Color::new(0.6, 0.5, 0.5, 1.0));
        }

        // Finish updating the physics engine
        engine.update();

        // End the game
        if is_key_down(KeyCode::Escape) {
            break 'running;
        }

        next_frame().await;
    }

    Ok(())

}