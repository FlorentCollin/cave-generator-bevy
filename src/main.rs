use bevy::prelude::*;
use rand::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CaveGeneratorPlugin)
        .run();
}

struct CaveGeneratorPlugin;

impl Plugin for CaveGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_cell)
            .add_startup_system(setup_camera)
            .add_system(change_colors);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct CellState {
    alive: bool,
}

impl Default for CellState {
    fn default() -> Self {
        CellState {
            alive: random::<bool>(),
        }
    }
}

//                               this query is for entities
//                               with a Sprite and CellState     Filter for the query to only run on modified
//                             👇components.                   👇entities with a  CellState component
fn change_colors(mut q: Query<(&mut Sprite, &CellState), Changed<CellState>>) {
    for (mut sprite, cell_state) in q.iter_mut() {
        sprite.color = if cell_state.alive {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }
}

fn spawn_cell(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(40.0, 40.0, 40.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CellState::default());
}
