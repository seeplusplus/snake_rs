use amethyst::ecs::*;
use amethyst::derive::SystemDesc;

use crate::components::Movement;
use crate::components::Direction;
use crate::components::Location;

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadStorage<'a, Movement>,
        WriteStorage<'a, Location>
    );

    fn run(&mut self, (mut locations, movements): Self::SystemData) {

    }
}