use bevy::{
    prelude::*
};
use crate::components::*;
fn round(value: f32) -> f32 {
    let floor = value.floor();
    let ceil = value.ceil();
    let mean = (floor + ceil) / 2.0;
    return if value >= mean { ceil } else { floor };
}

fn mouse_to_grid(coord: Vec2, step: f32, cellsX: f32, cellsY: f32) -> Vec2 {
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

        for i in 0..game_state.brush_size {
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
}
