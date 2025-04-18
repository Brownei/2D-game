use std::f32::consts::PI;

use raylib::{
    check_collision_circles,
    color::Color,
    math::Vector2,
    prelude::{KeyboardKey, MouseButton, RaylibDraw, RaylibDrawHandle},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Enemy {
    pub position: Vector2,
    pub size: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Bullet {
    pub position: Vector2,
    pub direction: Vector2,
    pub speed: f32,
    pub size: f32,
    pub pierce: i32,
}

pub struct Game {
    pub player_pos: Vector2,
    pub player_size: f32,
    pub player_speed: f32,
    pub shoot_delay: f32,
    pub game_time: f32,

    pub enemy_spawn_time: f32,
    pub enemy_count: i32,
    pub max_enemies: Vec<Enemy>,

    pub shoot_time: f32,
    pub bullet_speed: f32,
    pub bullet_size: f32,
    pub bullet_pierce: i32,
    pub bullet_count: i32,
    pub max_bullets: Vec<Bullet>,
}

impl Game {
    pub fn create_initial_state(
        width: i32,
        height: i32,
        max_enemies: usize,
        max_bullets: usize,
    ) -> Self {
        Game {
            player_pos: Vector2 {
                x: width as f32 / 2.0,
                y: height as f32 / 2.0,
            },
            game_time: 20.0,
            player_size: 30.0,
            player_speed: 1000.0,
            shoot_delay: 0.3,
            enemy_spawn_time: 0.0,
            shoot_time: 0.0,
            bullet_speed: 30.0,
            bullet_size: 15.0,
            bullet_pierce: 40,
            max_bullets: Vec::with_capacity(max_bullets),
            max_enemies: Vec::with_capacity(max_enemies),
            bullet_count: 0,
            enemy_count: 0,
        }
    }
}

pub fn shoot_bullets(
    d: &mut RaylibDrawHandle<'_>,
    game_state: &mut Game,
    fps: f32,
    max_bullets: usize,
) {
    let mut enemies_to_remove = Vec::new();
    if d.is_mouse_button_up(MouseButton::MOUSE_BUTTON_LEFT)
        && game_state.shoot_time < game_state.shoot_delay
    {
        game_state.shoot_time += fps
    }

    while d.is_mouse_button_up(MouseButton::MOUSE_BUTTON_LEFT)
        && game_state.shoot_time >= game_state.shoot_delay
    {
        if game_state.bullet_count >= max_bullets as i32 {
            break;
        }

        let direction = Vector2::new(
            d.get_mouse_x() as f32 - game_state.player_pos.x,
            d.get_mouse_y() as f32 - game_state.player_pos.y,
        )
        .normalized();

        let bullet = Bullet {
            position: game_state.player_pos,
            direction,
            speed: game_state.bullet_speed,
            size: game_state.bullet_size - 10.0,
            pierce: game_state.bullet_pierce,
        };

        game_state.bullet_count += 1;
        game_state.max_bullets.push(bullet);

        game_state.shoot_time -= game_state.shoot_delay;
    }

    for bullet in &mut game_state.max_bullets {
        bullet.position.x += bullet.direction.x * bullet.speed * fps;
        bullet.position.y += bullet.direction.y * bullet.speed * fps;

        let mut hit_enemy_index: Option<usize> = None;

        for (i, enemy) in game_state.max_enemies.iter().enumerate() {
            if check_collision_circles(enemy.position, enemy.size, bullet.position, bullet.size) {
                println!("I HIT SOMEONE");
                hit_enemy_index = Some(i);
                break;
            }
        }

        if let Some(index) = hit_enemy_index {
            game_state.max_enemies.remove(index);
            game_state.enemy_count -= 1;
        }

        d.draw_circle(
            bullet.position.x as i32,
            bullet.position.y as i32,
            bullet.size,
            Color::YELLOW,
        );

        if !enemies_to_remove.is_empty() {
            for &index in enemies_to_remove.iter() {
                if index < game_state.max_enemies.len() {
                    game_state.max_enemies.remove(index);
                }
            }
        }
    }
}

pub fn move_a_player(game_state: &mut Game, d: &mut RaylibDrawHandle<'_>, fps: f32) {
    if d.is_key_pressed(KeyboardKey::KEY_W) {
        game_state.player_pos.y -= game_state.player_speed * fps
    } else if d.is_key_pressed(KeyboardKey::KEY_A) {
        game_state.player_pos.x -= game_state.player_speed * fps
    } else if d.is_key_pressed(KeyboardKey::KEY_D) {
        game_state.player_pos.x += game_state.player_speed * fps
    } else if d.is_key_pressed(KeyboardKey::KEY_S) {
        game_state.player_pos.y += game_state.player_speed * fps
    }
}

pub fn spawn_enemies(
    d: &mut RaylibDrawHandle<'_>,
    game_state: &mut Game,
    max_enemies: usize,
    fps: f32,
    width: i32,
) {
    let spawn_frequency = 1.0 / game_state.game_time / 0.1;
    game_state.enemy_spawn_time += fps;

    while game_state.enemy_spawn_time >= spawn_frequency {
        // If max enemies reached, break out
        if game_state.enemy_count >= max_enemies as i32 {
            break;
        }

        let enemy_size = 8.0 * game_state.game_time / 20.0;
        let random_value: i32 = d.get_random_value(0..360);
        let radian = random_value as f32 * PI / 180.0;
        let direction: Vector2 = Vector2 {
            x: radian.cos(),
            y: radian.sin(),
        };

        let enemy = Enemy {
            size: enemy_size,
            position: Vector2 {
                x: game_state.player_pos.x + direction.x * width as f32 / 2.0,
                y: game_state.player_pos.y + direction.y * width as f32 / 2.0,
            },
        };

        game_state.enemy_count += 1;
        game_state.max_enemies.push(enemy);
        game_state.enemy_spawn_time -= spawn_frequency;
    }
}

pub fn update_enemies(game_state: &mut Game, d: &mut RaylibDrawHandle<'_>, fps: f32) {
    let enemy_speed = 40.0 + game_state.game_time / 10.0;
    for enemy in &mut game_state.max_enemies {
        let direction = Vector2::new(
            game_state.player_pos.x - enemy.position.x,
            game_state.player_pos.y - enemy.position.y,
        )
        .normalized();

        enemy.position.x += direction.x * enemy_speed * fps;
        enemy.position.y += direction.y * enemy_speed * fps;

        d.draw_circle(
            enemy.position.x as i32,
            enemy.position.y as i32,
            enemy.size,
            Color::RED,
        );
    }
}

pub fn gameover(
    game_state: &Game,
    enemy: &Enemy,
    width: i32,
    height: i32,
    max_enemies: usize,
    max_bullets: usize,
) {
    if check_collision_circles(
        enemy.position,
        enemy.size,
        game_state.player_pos,
        game_state.player_size,
    ) {
        println!("GAMEOVER!!!!!!");
        let _default_state: Game =
            Game::create_initial_state(width, height, max_enemies, max_bullets);
        //*game_state = Game::create_initial_state(width, height, max_enemies, max_bullets);
    }
}
