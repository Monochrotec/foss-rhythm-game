use bevy::{
    mesh::PrimitiveTopology, platform::collections::HashSet, prelude::*,
    shape::ConvexPolygonError::Concave,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup_camera, setup_notes))
        .run();
}

#[derive(Component, Clone)]
struct NoteColumn {
    /// the speed is a percentage of the height of the window that a note
    /// can trajj.
    speed: f32,

    /// entities of the notes and how far away they are from the hitpoint
    notes: HashSet<Entity>,
}

impl Default for NoteColumn {
    fn default() -> Self {
        Self {
            speed: 25.,
            notes: Default::default(),
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
            vec2(-75., 0.),
            vec2(-50., 25.),
            vec2(50., 25.),
            vec2(75., 0.),
            vec2(50., -25.),
            vec2(-50., -25.),
        ])
        .expect("The note shape is defined wrong"),
    );

    let note_material = materials.add(Color::WHITE);

    let notes_of_column_1 = [
        (1., NoteType::Tapping),
        (2., NoteType::Tapping),
        (3., NoteType::Holding(3.)),
        (7., NoteType::Tapping),
    ]
    .into_iter()
    .map(|(timestamp, note_type)| {
        commands
            .spawn((
                Mesh2d(note_shape.clone()),
                MeshMaterial2d(note_material.clone()),
                NoteTimestamp(timestamp),
                note_type,
            ))
            .id()
    })
    .collect();

    commands.spawn(NoteColumn {
        notes: notes_of_column_1,
        ..Default::default()
    });
}
