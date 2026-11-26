use bevy::{
    platform::collections::HashMap,
    prelude::*,
    settings::{ReflectSettingsGroup, SettingsGroup, SettingsPlugin},
};

pub struct MySettingsPlugin;

impl Plugin for MySettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SettingsPlugin::new("org.Monochrotec.foss-rhythm-game"));
    }
}

#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeybindAction {
    Pause,
    PressNote(u32),
}

#[derive(Resource, SettingsGroup, Reflect, Deref, DerefMut)]
#[reflect(Resource, SettingsGroup, Default)]
pub struct Keybinds(HashMap<KeyCode, KeybindAction>);

impl Default for Keybinds {
    fn default() -> Self {
        let binds = HashMap::from([
            (KeyCode::Escape, KeybindAction::Pause),
            (KeyCode::KeyA, KeybindAction::PressNote(0)),
            (KeyCode::KeyS, KeybindAction::PressNote(1)),
            (KeyCode::KeyD, KeybindAction::PressNote(2)),
            (KeyCode::KeyJ, KeybindAction::PressNote(3)),
            (KeyCode::KeyK, KeybindAction::PressNote(4)),
            (KeyCode::KeyL, KeybindAction::PressNote(5)),
        ]);

        Self(binds)
    }
}

impl Keybinds {
    pub fn is_keybind_pressed(&self, action: &KeybindAction, input: &ButtonInput<KeyCode>) -> bool {
        for keycode in input.get_pressed() {
            if self.get(keycode) == Some(action) {
                return true;
            }
        }

        false
    }

    pub fn is_keybind_just_pressed(
        &self,
        action: &KeybindAction,
        input: &ButtonInput<KeyCode>,
    ) -> bool {
        for keycode in input.get_just_pressed() {
            if self.get(keycode) == Some(action) {
                return true;
            }
        }

        false
    }
}
