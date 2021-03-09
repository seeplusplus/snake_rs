use amethyst::ecs::*;

use std::default::Default;

#[derive(Copy, Clone)]
pub struct Snake {
    pub length: u64
}

impl Snake {
    pub fn new() -> Snake {
        Snake::default()
    }
}

impl Default for Snake {
    fn default() -> Snake {
        Snake {
            length: 1
        }
    }
}

impl Component for Snake {
    type Storage = DenseVecStorage<Self>;

}

#[cfg(test)]
mod tests {
    use super::Snake;
 
    #[test]
    pub fn new_snake() {
        let s = Snake::new();
        println!("{}", s.length);

        assert!(true);
    }
}