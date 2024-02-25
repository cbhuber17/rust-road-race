// in src/main.rs
use rusty_engine::prelude::*;

const PLAYER_SPEED: f32 = 250.0;

// Custom game state struct
#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
}

fn main() {
    let mut game = Game::new();

    // Create the player sprite
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation.x = -500.0;
    player1.layer = 10.0;
    player1.collision = true;

    // Audio
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

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
}
