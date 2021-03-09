use amethyst::ecs::*;

pub enum Direction {
    None,
    Up,
    Right,
    Down,
    Left
}

/// Applied to entities that "move" freely.
/// As of now this would only be the head of the snake. 
/// The tail pieces do not "move," they are positioned linked
/// to other tail pieces or head pieces, so will follow the entities
/// they are attached to.
pub struct Movement {
    pub direction: Direction
}

impl Movement {
    pub fn new() -> Self {
        Movement {
            direction: Direction::None
        }
    }
}

impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}
