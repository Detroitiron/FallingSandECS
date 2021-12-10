use bevy::{core::FixedTimestep,
           prelude::*,
};

// Game State resource
pub struct GameState {
    pub game_paused: bool,
    pub game_step: bool,
    pub brush_size: usize,
    pub grid_height: f32,
    pub grid_width: f32,
    pub grid_step: f32,
    pub selected_particle: usize,
}
// Grid Resource
#[derive(Default)]
pub struct Grid(pub Vec<Vec<f32>>);

pub struct ParticleLocations {
    pub old_location: Vec2,
    pub new_location: Vec2,
}
#[derive(Debug, Clone, Copy)]
pub struct ParticleData {
    pub density: f32,
    pub particle_type: ParticleType,
    pub material: Color,
}
#[derive(Default)]
pub struct ParticleList(pub Vec<ParticleData>);
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ParticleType {
    Solid,
    Liquid,
    Gas,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum MyStage {
    BeforeMove,
    AfterMove,
}
// CONSTANT variables
const GRID_HEIGHT: usize = 30;
const GRID_WIDTH: usize = 60;
const GRID_STEP: usize = 30;
fn keyboard_system(
    mut game_state: ResMut<GameState>,
    keyboard_input: Res<Input<KeyCode>>
) {
    if keyboard_input.pressed(KeyCode::Key1) {
        game_state.selected_particle = 0;
    } else if keyboard_input.pressed(KeyCode::Key2) {
        game_state.selected_particle = 1;
    } else if keyboard_input.pressed(KeyCode::Key3) {
        game_state.selected_particle = 2;
    } else if keyboard_input.pressed(KeyCode::Key4) {
        game_state.selected_particle = 3;
    }
}

fn startup_system(mut commands: Commands, mut grid: ResMut<Grid>,
) {
    // let mut camera = OrthographicCameraBundle::new_2d();
    // let screenX = (GRID_HEIGHT as f32 * GRID_STEP as f32) / 2 as f32;
    // let screenY = (GRID_WIDTH as f32 * GRID_STEP as f32) / 2 as f32;
    // camera.orthographic_projection.scale = 3.0;
    // camera.transform = Transform::from_xyz(screenX, screenY, 0.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    let grid_height = GRID_HEIGHT as f32;
    let grid_width = GRID_WIDTH as f32;
    let step = GRID_STEP as f32;
    commands.insert_resource(GameState {
        game_step: false,
        game_paused: false,
        brush_size: 1,
        grid_height,
        grid_width,
        grid_step: step,
        selected_particle: 2,
    });
    let particle_list = vec![
        ParticleData {
            density: 1.0,
            particle_type: ParticleType::Liquid,
            material: Color::BLUE,
        },
        ParticleData {
            density: 0.881,
            particle_type: ParticleType::Liquid,
            material: Color::YELLOW,
        },
        ParticleData {
            density: 2.6,
            particle_type: ParticleType::Solid,
            material: Color::DARK_GRAY,
        },
        ParticleData {
            density: 0.00059,
            particle_type: ParticleType::Gas,
            material: Color::ALICE_BLUE,
        },
    ];

    commands.insert_resource(ParticleList(particle_list));
    grid.0.append(&mut vec![
        vec![0.0; GRID_HEIGHT + 1];
        GRID_WIDTH + 1
    ])
}

fn calc_next_system(
    mut query: Query<(&ParticleData, &mut ParticleLocations, &Transform)>,
    game_state: Res<GameState>,
    mut grid: ResMut<Grid>,
) {
    for (particle_data, mut particle_locations, _transforms) in query.iter_mut() {
        let location = particle_locations.old_location;
        let particle_type = particle_data.particle_type;
        let v = match particle_type {
            ParticleType::Solid => vec![
                Vec2::new(0.0, -1.0),
                Vec2::new(-1.0, -1.0),
                Vec2::new(1.0, -1.0),
            ],
            ParticleType::Liquid => vec![
                Vec2::new(0.0, -1.0),
                Vec2::new(-1.0, -1.0),
                Vec2::new(1.0, -1.0),
                Vec2::new(-1.0, 0.0),
                Vec2::new(1.0, 0.0),
            ],
            ParticleType::Gas => vec![
                Vec2::new(0.0, 1.0),
                Vec2::new(-1.0, 1.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(-1.0, 0.0),
                Vec2::new(1.0, 0.0),
            ],
        };
        let mut changed = false;
        for loc in v.iter() {
            let modified_loc = location + *loc;
            let valid_check = !(modified_loc.x < 0.0)
                && !(modified_loc.x > game_state.grid_width)
                && !(modified_loc.y < 0.0)
                && !(modified_loc.y > game_state.grid_height);
            if valid_check {
                let neighbor_particle_data = grid.0[modified_loc.x as usize][modified_loc.y as usize].clone();
                if neighbor_particle_data == 0.0
                {
                    changed = true;
                    particle_locations.new_location = modified_loc;
                    grid.0[modified_loc.x as usize][modified_loc.y as usize] = particle_data.density;
                    grid.0[location.x as usize][location.y as usize] = 0.0;
                    break;
                }
            }
        }
        if !changed {
            particle_locations.new_location = location;
        }
    }
}

fn move_next_system(
    mut query: Query<(&mut ParticleLocations, &mut Transform), Changed<ParticleLocations>>,
    game_state: Res<GameState>,
) {
    let step = game_state.grid_step;
    for (mut particle_locations, mut transform) in query.iter_mut() {
        let grid_x = game_state.grid_width / 2.0;
        let grid_y = game_state.grid_height / 2.0;

        let changed_location =
            (particle_locations.new_location - Vec2::new(grid_x, grid_y)) * Vec2::splat(step);

        transform.translation = changed_location.extend(0.0);
        particle_locations.old_location = particle_locations.new_location;
    }

}


fn round(value: f32) -> f32 {
    let floor = value.floor();
    let ceil = value.ceil();
    let mean = (floor + ceil) / 2.0;
    return if value >= mean { ceil } else { floor };
}

fn mouse_to_grid(coord: Vec2, step: f32, cells_x: f32, cells_y: f32) -> Vec2 {
    let x_value = round(coord.x / step - 1.0);
    let y_value = round(coord.y / step - 1.);
    let grid_x = cells_x / 2.0;
    let grid_y = cells_y / 2.0;
    let mut x = (x_value - grid_x) * step;
    let mut y = (y_value - grid_y) * step;
    let board_x = grid_x * step;
    let board_y = grid_y * step;
    if x >= board_x {
        x = board_x;
    } else if x <= -board_x {
        x = -board_x;
    }
    if y >=board_y {
        y = board_y;
    } else if y <= -board_y {
        y = -board_y;
    }

    Vec2::new(x, y)
}

// Screen 0,0 is at the bottom left of the window. 0,0 is the
pub fn mouse_click_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut grid: ResMut<Grid>,
    particle_list: Res<ParticleList>,
) {
    let win = match windows
        .get_primary()
        .expect("No primary window")
        .cursor_position()
    {
        Some(v) => v,
        None => Vec2::ZERO,
    };
    if mouse_button_input.pressed(MouseButton::Left) {
        let particle_data = particle_list.0[game_state.selected_particle];

        let translation = mouse_to_grid(
            win,
            game_state.grid_step,
            game_state.grid_width,
            game_state.grid_height,
        )
            .extend(0.0);
        let location = (translation.truncate() / Vec2::splat(game_state.grid_step))
            + Vec2::new(game_state.grid_width / 2.0, game_state.grid_height / 2.0);

        if grid.0[location.x as usize][location.y as usize] == 0.0 {
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(particle_data.material.into()),
                    transform: Transform::from_translation(translation),
                    sprite: Sprite::new(Vec2::splat(game_state.grid_step)),
                    ..Default::default()
                })
                .insert(particle_data)
                .insert(ParticleLocations {
                    old_location: location,
                    new_location: location,
                });
            grid.0[location.x as usize][location.y as usize] = particle_data.density;
        }
    }
}

// Add a before system to calculate the new_location for all of the particles.
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: (GRID_WIDTH * GRID_STEP + 2 * GRID_STEP) as f32,
            height: (GRID_HEIGHT * GRID_STEP + 2 * GRID_STEP) as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<Grid>()
        .add_startup_system(startup_system.system())
        .add_system(mouse_click_system.system())
        .add_system(keyboard_system.system())
        .add_stage_before(
            CoreStage::Update,
            MyStage::BeforeMove,
            SystemStage::parallel().with_run_criteria(FixedTimestep::step(0.1)),
        )
        .add_stage_after(
            CoreStage::Update,
            MyStage::AfterMove,
            SystemStage::parallel().with_run_criteria(FixedTimestep::step(0.1)),
        )
        .add_system_to_stage(MyStage::BeforeMove, calc_next_system.system())
        .add_system_to_stage(MyStage::AfterMove, move_next_system.system())
        .run();
}
