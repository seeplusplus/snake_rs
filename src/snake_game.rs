
use amethyst::{assets::{AssetStorage, Loader, Handle}, core::transform::Transform, ecs::{Component, DenseVecStorage, world}, prelude::*, renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, formats::texture}};

use crate::snake::Snake;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct SnakeGame;

impl SimpleState for SnakeGame {
    fn on_start(&mut self, data: StateData<'_ ,GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Snake>();

        let sprite_sheet_handle = load_sprite_sheet(world);
        
        initialize_snake(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}

fn initialize_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_snake(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut snake_transform = Transform::default();
    
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    snake_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Snake::new())
        .with(snake_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let loader = world.read_resource::<Loader>();
    
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/rustacean-flat-happy.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/rustacean-flat-happy.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
