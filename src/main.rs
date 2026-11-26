mod settings;

use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
    time::Stopwatch,
};

use crate::settings::{KeybindAction, Keybinds, MySettingsPlugin};

const WINDOW_HEIGHT: f32 = 1080.;

/// Really hard to explain for me what i did but the hit place is
/// like if you divide the window height to three sections and decide to
/// make the line between the middle and bottom section and use that as a hit place.
///
/// it is subtracted by half of the window height because the camera is
/// focusing on the center point
const HIT_PLACE: f32 = WINDOW_HEIGHT / 3. - WINDOW_HEIGHT / 2.;

const DEFAULT_NOTE_MOVEMENT_SPEED: f32 = 150.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MySettingsPlugin,
        ))
        .init_resource::<InLevelTime>()
        .init_resource::<InLevelTime>()
        .init_resource::<PressableNotes>()
        .init_resource::<MissedNotes>()
        .add_systems(Startup, (setup_camera, setup_hit_place, setup_notes))
        .add_systems(
            FixedUpdate,
            (update_note_translation, check_for_note_presses).chain(),
        )
        .run();
}

#[derive(Resource, Default)]
struct InLevelTime(Stopwatch);

#[derive(Resource, Default)]
struct PressableNotes(HashMap<u32, Entity>);

#[derive(Resource, Default)]
struct MissedNotes(HashSet<Entity>);

#[derive(Component)]
struct NoteColumn {
    id: u32,
    speed: f32,
}

#[derive(Component)]
struct NoteTimestamp(f32);

#[derive(Component)]
enum NoteType {
    Tapping,
    Holding(f32),
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_hit_place(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let hit_place_shape = meshes.add(
        ConvexPolygon::new([
            vec2(-350., 0.),
            vec2(-320., 2.5),
            vec2(320., 2.5),
            vec2(350., 0.),
            vec2(320., -2.5),
            vec2(-320., -2.5),
        ])
        .expect("the hit place shape is defined wrong"),
    );

    let hit_place_material = materials.add(Color::WHITE);

    commands.spawn((
        Transform::from_xyz(0., HIT_PLACE, 0.),
        Mesh2d(hit_place_shape),
        MeshMaterial2d(hit_place_material),
    ));
}

fn setup_notes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // defines the shape of the note.
    //
    // which is a wide hexagon.
    let note_shape = meshes.add(
        ConvexPolygon::new([
            vec2(-40., 0.),
            vec2(-30., 15.),
            vec2(30., 15.),
            vec2(40., 0.),
            vec2(30., -15.),
            vec2(-30., -15.),
        ])
        .expect("The note shape is defined wrong"),
    );

    let note_material = materials.add(Color::WHITE);

    commands
        .spawn((
            Transform::default(),
            NoteColumn {
                id: 0,
                speed: DEFAULT_NOTE_MOVEMENT_SPEED,
            },
            Visibility::Visible,
        ))
        .with_children(|child_cmd| {
            [
                (1., NoteType::Tapping),
                (2., NoteType::Tapping),
                (3., NoteType::Holding(3.)),
                (7., NoteType::Tapping),
            ]
            .into_iter()
            .for_each(|(timestamp, note_type)| {
                child_cmd.spawn((
                    Transform::default(),
                    Mesh2d(note_shape.clone()),
                    MeshMaterial2d(note_material.clone()),
                    NoteTimestamp(timestamp),
                    note_type,
                ));
            });
        });
}

fn update_note_translation(
    time: Res<Time>,
    mut in_level_time: ResMut<InLevelTime>,

    column_query: Query<(&Children, &NoteColumn)>,
    mut note_query: Query<(&mut Transform, &NoteTimestamp)>,
) {
    in_level_time.0.tick(time.delta());

    for (children, column) in column_query {
        for child in children {
            if let Ok((mut note_transform, note_timestamp)) = note_query.get_mut(*child) {
                note_transform.translation.y =
                    (note_timestamp.0 - in_level_time.0.elapsed_secs()) * column.speed + HIT_PLACE;
            }
        }
    }
}

fn check_for_note_presses(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    keybinds: Res<Keybinds>,

    mut in_level_time: ResMut<InLevelTime>,
    mut missed_notes: ResMut<MissedNotes>,

    column_query: Query<(&Children, &NoteColumn)>,
    note_query: Query<&Transform, (With<NoteTimestamp>, With<NoteType>)>,
) {
    in_level_time.0.tick(time.delta());

    for (children, column) in column_query {
        let is_just_pressed =
            keybinds.is_keybind_just_pressed(&KeybindAction::PressNote(column.id), &input);

        for child in children {
            if let Ok(note_transform) = note_query.get(*child) {
                if (note_transform.translation.y - HIT_PLACE).abs() <= 20. && is_just_pressed {
                    println!("Note for column {} just pressed", column.id);
                }
            }
        }
    }
}
