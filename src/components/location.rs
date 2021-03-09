use amethyst::ecs::*;

/// Represent locations within the game
pub enum Location {
    /// Items can have free positioning e.g.,
    /// + Pips
    /// + Head of snake
    Free {x: u32, y: u32 },
    /// or linked, such as the links in the snake
    Linked(Box<Location>)
}

impl Component for Location {
    type Storage = DenseVecStorage<Self>;
}