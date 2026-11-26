mod file;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    WINDOW_HEIGHT,
    settings::{KeybindAction, Keybinds},
};

/// Really hard to explain for me what i did but the hit place is
/// like if you divide the window height to three sections and decide to
/// make the line between the middle and bottom section and use that as a hit place.
///
/// it is subtracted by half of the window height because the camera is
/// focusing on the center point
const HIT_PLACE: f32 = WINDOW_HEIGHT / 3. - WINDOW_HEIGHT / 2.;

const DEFAULT_NOTE_MOVEMENT_SPEED: f32 = 150.;

pub struct MyLevelPlugin;

impl Plugin for MyLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<WrongPress>()
            .init_resource::<InLevelTime>()
            .init_resource::<InLevelTime>()
            .add_systems(Startup, (setup_camera, setup_hit_place, setup_notes))
            .add_systems(
                FixedUpdate,
                (
                    update_note_translation,
                    check_for_note_presses,
                    despawn_pressed_notes,
                )
                    .chain(),
            );
    }
}
/// this is for when u press while there isnt any notes in range
#[derive(Message)]
struct WrongPress;

#[derive(Resource, Default)]
struct InLevelTime(Stopwatch);

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

#[derive(Component, PartialEq, Eq)]
enum NoteState {
    None,
    Pressed,
    Missed,
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
                    NoteState::None,
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
    mut wrong_press_writer: MessageWriter<WrongPress>,

    input: Res<ButtonInput<KeyCode>>,
    keybinds: Res<Keybinds>,

    column_query: Query<(&Children, &NoteColumn)>,
    mut note_query: Query<(&mut NoteState, &Transform), With<NoteType>>,
) {
    for (children, column) in column_query {
        let is_just_pressed =
            keybinds.is_keybind_just_pressed(&KeybindAction::PressNote(column.id), &input);

        let mut is_any_note_in_range = false;

        for child in children {
            if let Ok((mut note_state, note_transform)) = note_query.get_mut(*child) {
                let note_hitplace_diff = note_transform.translation.y - HIT_PLACE;

                if note_hitplace_diff.abs() <= 17.5 && is_just_pressed {
                    is_any_note_in_range = true;

                    if is_just_pressed {
                        *note_state = NoteState::Pressed;
                    }
                }

                if note_hitplace_diff < -17.5 && *note_state != NoteState::Pressed {
                    *note_state = NoteState::Missed;
                }
            }
        }

        // if there isnt any note in range and the key for the column is pressed send a
        // message that a wrong press has happened.
        if !is_any_note_in_range && is_just_pressed {
            wrong_press_writer.write(WrongPress);
        }
    }
}

fn despawn_pressed_notes(mut commands: Commands, query: Query<(Entity, &NoteState)>) {
    for (entity, note_state) in query {
        if *note_state == NoteState::Pressed {
            commands.entity(entity).despawn();
        }
    }
}
