// in src/main.rs
use rand::prelude::*;
use rusty_engine::prelude::*;

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

// Custom game state struct
#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
}

fn add_player<'game>(game: &'game mut Game<GameState>, player_name: &'game str, car: SpritePreset) {
    let player = game.add_sprite(player_name, car);
    player.translation.x = -500.0;
    player.layer = 10.0;
    player.collision = true;
}

fn main() {
    let mut game = Game::new();

    // Create the player sprite, name is restricted to "playerX"
    add_player(&mut game, "player1", SpritePreset::RacingCarBlue);

    // Audio
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // Create the road lines
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    // Road obstacles
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
    ];

    // Init obstacles
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    // Create the health message
    let health_message = game.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    // Add one or more functions with logic for your game. When the game is run, the logic
    // functions will run in the order they were added.
    game.add_logic(game_logic);

    // Run the game, with an initial state
    let initial_game_state = GameState {
        health_amount: 5,
        lost: false,
    };

    game.run(initial_game_state);
}

// The first parameter is always a `&mut Engine`, and the second parameter is a mutable reference to your custom game state struct (`&mut GameState` in this case).
// This function will be run once each frame.
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Don't run any more game logic if the game has ended
    if game_state.lost {
        return;
    }

    // Collect keyboard input
    let mut direction = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    // Move/rotate the player sprite
    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;

    // End the game when OOB
    if player1.translation.y < -360.0 || player1.translation.y > 360.0 {
        game_state.health_amount = 0;
    }

    // Move road objects
    for sprite in engine.sprites.values_mut() {
        // Road lines
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;

            // Translate road objects to other side of screen if gone too far
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }

        // Obstacles
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }

    // Deal with collisions
    let health_message = engine.texts.get_mut("health_message").unwrap();

    // Go through all collision events and act accordingly
    for event in engine.collision_events.drain(..) {
        // We don't care if obstacles collide with each other or collisions end
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.7);
        }
    }

    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.75);
    }
}
