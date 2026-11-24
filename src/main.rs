use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup_camera, setup_notes))
        .add_systems(Update, update_note_translation)
        .run();
}

#[derive(Component, Clone)]
struct NoteColumnSettings {
    speed: f32,
}

impl Default for NoteColumnSettings {
    fn default() -> Self {
        Self { speed: 50. }
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
            NoteColumnSettings::default(),
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
    column_query: Query<(&Children, &NoteColumnSettings)>,
    mut note_query: Query<(&mut Transform, &NoteTimestamp)>,
) {
    for (children, column_settings) in column_query {
        for child in children {
            if let Ok((mut note_transform, note_timestamp)) = note_query.get_mut(*child) {
                note_transform.translation.y = note_timestamp.0 * column_settings.speed;
            }
        }
    }
}
