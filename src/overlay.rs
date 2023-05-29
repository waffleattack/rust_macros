//! Shows how to display a window in transparent mode.
//!
//! This feature works as expected depending on the platform. Please check the
//! [documentation](https://docs.rs/bevy/latest/bevy/prelude/struct.WindowDescriptor.html#structfield.transparent)
//! for more details.
use std::sync::*;
use std::sync::mpsc::*;

use lazy_static::lazy_static;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{Window, WindowLevel, WindowPlugin, WindowResolution},
};
#[cfg(target_os = "macos")]
use bevy::window::CompositeAlphaMode;
use types::DisplayInfo;

use crate::types;

lazy_static! {

    pub static ref MY_CHANNEL: (Mutex<Receiver<DisplayInfo>>, Mutex<Sender<DisplayInfo>>) = {
        let (rx, tx) = channel();
        (Mutex::new(tx),Mutex::new(rx))
    };
}


pub fn init_screen() {
    App::new()
        // ClearColor must have 0 alpha, otherwise some color will bleed through
        .insert_resource(ClearColor(Color::NONE))
        .add_startup_system(setup)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(text_update_system)
        .add_system(text_color_system)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Setting `transparent` allows the `ClearColor`'s alpha value to take effect
                transparent: true,
                // Disabling window decorations to make it feel more like a widget than a window
                decorations: true,
                window_level: WindowLevel::AlwaysOnTop,
                resizable: true,
                resolution: WindowResolution::new(500.0, 50.0),
                #[cfg(target_os = "macos")]
                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                ..default()
            }),
            ..default()
        }))
        .run();
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Mode: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            }),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 30.0,
                color: Color::GOLD,
            }),
        ]),
        FpsText,
    ));
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}

fn text_update_system(mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Ok(val) = &MY_CHANNEL.0.lock().unwrap().try_recv() {
            text.sections[1].value = val.mode.clone();
            if let Some(v) = val.current_action.clone() {
                text.sections[2].value = format!("\n{}", v);
            }
        }
    }
}