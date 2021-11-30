use bevy::{
    prelude::*,
};

// Game State resource
struct GameState {
    game_paused: bool,
    game_step: bool,
    brush_size: usize,
    grid_height: f32,
    grid_width: f32,
    grid_step: f32,
    selected_particle: usize
}
// Grid Resource
#[derive(Default)]
struct Grid (Vec<Vec<usize>>);

struct ParticleLocations {
    old_location: Vec2,
    new_location: Vec2,
}
#[derive(Debug, Clone, Copy)]
struct ParticleData {
    density: f32,
    particle_type: ParticleType,
    material: Color
}
#[derive(Default)]
struct ParticleList (Vec<ParticleData>);
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum ParticleType {
    Solid,
    Liquid,
    Gas
}
const GRID_HEIGHT: usize = 30;
const GRID_WIDTH: usize = 30;
const GRID_STEP: usize = 30;
const GRID_OFFSET: usize = 2;
fn startup_system(mut commands: Commands, mut grid: ResMut<Grid>) {
    // let mut camera = OrthographicCameraBundle::new_2d();
    // let screenX = (GRID_HEIGHT as f32 * GRID_STEP as f32) / 2 as f32;
    // let screenY = (GRID_WIDTH as f32 * GRID_STEP as f32) / 2 as f32;
    // camera.orthographic_projection.scale = 3.0;
    // camera.transform = Transform::from_xyz(screenX, screenY, 0.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    let grid_height = 30.;
    let grid_width = 30.;
    let step = 30.;
    commands.insert_resource(GameState {
        game_step: false,
        game_paused: false,
        brush_size: 1,
        grid_height,
        grid_width,
        grid_step: step,
        selected_particle: 0
    });
    commands.insert_resource(ParticleList(vec![
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
            material: Color::rgb(0.4, 0.2, 0.8),
        },
        ParticleData {
            density: 0.00059,
            particle_type: ParticleType::Gas,
            material: Color::ALICE_BLUE,
        }
    ]))


}
fn round (value:f32) -> f32 {
    let floor = value.floor();
    let ceil = value.ceil();
    let mean = (floor + ceil) / 2.0;
    return if value >= mean {
        ceil
    } else {
        floor
    }
}
fn mouse_to_grid (coord: Vec2, step: f32, cellsX: f32, cellsY: f32) -> Vec2 {
    let x_value = round(coord.x / step);
    let y_value = round(coord.y / step);
    let grid_x = cellsX * step;
    let grid_y = cellsY * step;
    let board_x = grid_x / 2.0;
    let board_y = grid_y / 2.0;
    let mut x = x_value * step - board_x;
    let mut y = y_value * step - board_y;

    if x >= board_x {
        x = board_x - step;
    } else if x <= -board_x {
        x = -board_x + step;
    }
    if y >= board_y {
        y = board_y - step;
    } else if y <= -board_y {
        y = -board_y + step;
    }

    println!("{}, {}\n{:?}", x, y, coord);

    Vec2::new(x, y)

}
// Screen 0,0 is at the bottom left of the window. 0,0 is the
fn mouse_click_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    particle_list: Res<ParticleList>) {
    let win = match windows.get_primary().expect("No primary window").cursor_position() {
        Some(v) => v,
        None => Vec2::ZERO,
    };
    if mouse_button_input.pressed(MouseButton::Left) {
        let particle_data = particle_list.0[game_state.selected_particle];
        println!("ParticleData {:?}", particle_data);

        for i in 0..game_state.brush_size {
            let translation = mouse_to_grid(win, game_state.grid_step, game_state.grid_width, game_state.grid_height).extend(0.0);
            let location = translation.truncate() / Vec2::splat(game_state.grid_step);
            commands.spawn_bundle(SpriteBundle {
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
        }
    }
}
// Add a before system to calculate the new_location for all of the particles.
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 900.,
            height: 900.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<Grid>()
        .add_startup_system(startup_system.system())
        .add_system(mouse_click_system.system())
        .run();
}