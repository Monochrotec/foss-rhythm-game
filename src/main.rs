use bevy::{prelude::*, time::Stopwatch};

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
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<InLevelTime>()
        .add_message::<NotePressed>()
        .add_systems(Startup, (setup_camera, setup_hit_place, setup_notes))
        .add_systems(Update, (check_for_note_press, update_note_translation))
        .run();
}

#[derive(Resource, Default)]
struct InLevelTime(Stopwatch);

#[derive(Component, Clone)]
struct NoteColumn {
    id: u32,
    speed: f32,
    time_range: f32,
}

impl NoteColumn {
    fn new(id: u32, speed: f32) -> Self {
        Self {
            id,
            speed,
            time_range: 2.5 / speed,
        }
    }
}

#[derive(Component, Clone, Default)]
struct NoteTimestamp(f32);

#[derive(Component, Clone, Default)]
enum NoteType {
    #[default]
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
            vec2(-30., 10.),
            vec2(30., 10.),
            vec2(40., 0.),
            vec2(30., -10.),
            vec2(-30., -10.),
        ])
        .expect("The note shape is defined wrong"),
    );

    let note_material = materials.add(Color::WHITE);

    commands
        .spawn((
            Transform::default(),
            NoteColumn::new(0, DEFAULT_NOTE_MOVEMENT_SPEED),
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
    in_level_time: Res<InLevelTime>,

    column_query: Query<(&Children, &NoteColumn)>,
    mut note_query: Query<(&mut Transform, &NoteTimestamp)>,
) {
    for (children, column) in column_query {
        for child in children {
            if let Ok((mut note_transform, note_timestamp)) = note_query.get_mut(*child) {
                note_transform.translation.y =
                    (note_timestamp.0 - in_level_time.0.elapsed_secs()) * column.speed + HIT_PLACE;
            }
        }
    }
}

#[derive(Message)]
struct NotePressed(Entity);

fn check_for_note_press(
    mut note_action_writer: MessageWriter<NotePressed>,

    time: Res<Time>,
    mut in_level_time: ResMut<InLevelTime>,

    column_query: Query<(&Children, &NoteColumn)>,
    note_query: Query<&NoteTimestamp>,
) {
    in_level_time.0.tick(time.delta());

    for (children, column) in column_query {
        for child in children {
            if let Ok(note_timestamp) = note_query.get(*child) {
                if (note_timestamp.0 - in_level_time.0.elapsed_secs()).abs() <= column.time_range {
                    note_action_writer.write(NotePressed(*child));

                    println!("hovering over note with entity id {child}");
                }
            }
        }
    }
}
