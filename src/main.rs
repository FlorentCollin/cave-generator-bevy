use bevy::prelude::*;
use rand::random;

const CAVE_WIDTH: i32 = 50;
const CAVE_HEIGHT: i32 = 50;
const CELL_SIZE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CaveGeneratorPlugin)
        .run();
}

struct CaveGeneratorPlugin;

impl Plugin for CaveGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new())
            .add_startup_system(Board::spawn_cells)
            .add_startup_system(setup_camera)
            .add_system(change_colors)
            .add_system(move_cell);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
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

fn spawn_cell(commands: &mut Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE * 0.8, CELL_SIZE * 0.8, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CellState::default())
        .insert(position)
        .id()
}

fn move_cell(mut q: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            position.x as f32 * CELL_SIZE - (CAVE_WIDTH as f32 / 2.0) * CELL_SIZE,
            position.y as f32 * CELL_SIZE - (CAVE_HEIGHT as f32 / 2.0) * CELL_SIZE,
            0.0);
    }

}

struct Board {
    cells: Vec<Entity>,
}

impl Board {
    pub fn new() -> Self {
        Self { cells: vec![] }
    }

    pub fn spawn_cells(mut commands: Commands, mut board: ResMut<Board>) {
        for i in 0..CAVE_WIDTH {
            for j in 0..CAVE_HEIGHT {
                let entity = spawn_cell(
                    &mut commands,
                    Position {
                        x: i,
                        y: j,
                    },
                );
                board.cells.push(entity);
            }
        }
    }
}
