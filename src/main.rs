//! Pong Tutorial 1

mod components;
mod systems;
mod snake_game;

use snake_game::SnakeGame;

use amethyst::{core::TransformBundle, prelude::*, renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    }, utils::application_root_dir};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([1.0, 1.0, 1.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(systems::MovementSystem, "movement_system", &[]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, SnakeGame, game_data)?;
    game.run();

    Ok(())
}


