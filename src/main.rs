use bevy::prelude::*;
use rand::random;

const CAVE_WIDTH: i32 = 80;
const CAVE_HEIGHT: i32 = 80;
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
            .add_system(Board::update)
            .add_system(change_colors)
            .add_system(move_cell)
            .add_system(restart);
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
            0.0,
        );
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
                let entity = spawn_cell(&mut commands, Position { x: i, y: j });
                board.cells.push(entity);
            }
        }
    }

    pub fn update(board: Res<Board>, mut cells_states_query: Query<(&mut CellState, &Position)>) {
        let cells_states: Vec<_> = board
            .cells
            .iter()
            .map(|cell_entity| {
                let (cell_state, _ ) = cells_states_query.get(*cell_entity).unwrap();
                cell_state
            })
            .collect();

        for (mut cell_state, position) in cells_states_query.iter_mut() {
            let neighbors_alive_count = board.count_neighbors_alive(&cells_states, position);

            cell_state.alive = if cell_state.alive {
                neighbors_alive_count > 3
            } else {
                neighbors_alive_count > 4
            };
        }
    }

    fn count_neighbors_alive(
        &self,
        cells_states: &Vec<&CellState>,
        position: &Position,
    ) -> i32 {
        // count the number of neighbors alive
        let mut neighbors_alive_count = 0;
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let index = (position.y + j) * CAVE_WIDTH + (position.x + i);
                if let Some(cell_state) = cells_states.get(index as usize) {
                    if cell_state.alive {
                        neighbors_alive_count += 1;
                    }
                }
            }
        }
        neighbors_alive_count
    }
}

fn restart(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut cell_states_query: Query<&mut CellState>,
) {
    // restart the simulation
    if keyboard_input.pressed(KeyCode::R) || mouse_input.pressed(MouseButton::Left) {
        for mut cell_state in cell_states_query.iter_mut() {
            *cell_state = CellState::default();
        }
    }
}
