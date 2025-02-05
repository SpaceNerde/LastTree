use bevy::{prelude::*, window::WindowMode};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Windowed,
                        fit_canvas_to_parent: false,
                        title: "Last Tree".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .run();
}
