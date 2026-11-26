mod level;
mod settings;

use bevy::prelude::*;

use crate::{level::MyLevelPlugin, settings::MySettingsPlugin};

const WINDOW_HEIGHT: f32 = 1080.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MyLevelPlugin,
            MySettingsPlugin,
        ))
        .run();
}
