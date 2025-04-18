use game::{gameover, move_a_player, shoot_bullets, spawn_enemies, update_enemies, Game};
use raylib::prelude::*;

mod game;
mod logic;

fn main() {
    const WINDOW_HEIGHT: i32 = 480;
    const WINDOW_WIDTH: i32 = 800;
    const MAX_BULLETS: usize = 100;
    const MAX_ENEMIES: usize = 100;
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, World")
        .vsync()
        .build();
    rl.set_target_fps(60);

    //Initiliaze game state
    let mut game_state =
        Game::create_initial_state(WINDOW_WIDTH, WINDOW_HEIGHT, MAX_ENEMIES, MAX_BULLETS);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let fps = d.get_frame_time();

        d.clear_background(Color::BLACK);

        //DRAW PLAYER
        d.draw_circle(
            game_state.player_pos.x as i32,
            game_state.player_pos.y as i32,
            game_state.player_size,
            Color::GREEN,
        );

        shoot_bullets(&mut d, &mut game_state, fps, MAX_BULLETS);
        //
        ////MOVEMENT
        move_a_player(&mut game_state, &mut d, fps);
        //
        ////ENEMIES SPAWNING
        spawn_enemies(&mut d, &mut game_state, MAX_ENEMIES, fps, WINDOW_WIDTH);
        //
        update_enemies(&mut game_state, &mut d, fps);

        for enemy in &game_state.max_enemies {
            gameover(
                &game_state,
                enemy,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                MAX_ENEMIES,
                MAX_BULLETS,
            );
        }
    }
}
