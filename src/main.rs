use rand::prelude::*;
use rusty_engine::prelude::*;

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

// Custom game state struct
#[derive(Resource)]
struct GameState<'name> {
    player_name: &'name str,
    health_amount: u8,
    lost: bool,
}

// ----------------------------------------------------------------------------------------------

// Main helpers

// ----------------------------------------------------------------------------------------------

fn add_player<'game>(game: &'game mut Game<GameState>, player_name: &'game str, car: SpritePreset) {
    let player = game.add_sprite(player_name, car);
    player.translation.x = -500.0;
    player.layer = 10.0;
    player.collision = true;
}

// ----------------------------------------------------------------------------------------------

fn set_game_audio(game: &mut Game<GameState>, music: MusicPreset, volume: f32) {
    game.audio_manager.play_music(music, volume);
}

// ----------------------------------------------------------------------------------------------

fn create_road_lines(game: &mut Game<GameState>, barrier: SpritePreset) {
    const NUM_LINES: u8 = 10;

    for i in 0..NUM_LINES {
        let roadline = game.add_sprite(format!("roadline{}", i), barrier);
        roadline.scale = 0.2;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }
}

// ----------------------------------------------------------------------------------------------

fn add_obstacles(game: &mut Game<GameState>) {
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
}

// ----------------------------------------------------------------------------------------------

fn create_message(game: &mut Game<GameState>, label: &str, text: &str, x: f32, y: f32) {
    let message = game.add_text(label, text);
    message.translation = Vec2::new(x, y);
}

// ----------------------------------------------------------------------------------------------

fn main() {
    let mut game = Game::new();
    const PLAYER_NAME: &str = "Colin";
    let initial_game_state = GameState {
        player_name: PLAYER_NAME,
        health_amount: 5,
        lost: false,
    };

    // Create the player sprite
    add_player(&mut game, PLAYER_NAME, SpritePreset::RacingCarBlue);

    // Show player name on top left
    create_message(&mut game, "player_message", PLAYER_NAME, -550.0, 320.0);

    // Audio
    set_game_audio(&mut game, MusicPreset::WhimsicalPopsicle, 0.2);

    // Road lines
    create_road_lines(&mut game, SpritePreset::RacingBarrierWhite);

    // Road obstacles
    add_obstacles(&mut game);

    // Health info on top right
    create_message(
        &mut game,
        "health_message",
        &format!("Health: {}", initial_game_state.health_amount),
        550.0,
        320.0,
    );

    // Add one or more functions with logic for the game. When the game is run, the logic
    // functions will run in the order they were added.
    game.add_logic(game_logic);

    // Run the game, with an initial state
    game.run(initial_game_state);
}

// ----------------------------------------------------------------------------------------------

// Game Logic Helpers

// ----------------------------------------------------------------------------------------------

fn handle_keyboard(engine: &mut Engine, game_state: &mut GameState) {
    let mut direction = 0.0;

    // React to keyboard input
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }

    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    // Move/rotate the player sprite
    let player = engine.sprites.get_mut(game_state.player_name).unwrap();
    player.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player.rotation = direction * 0.15;

    // End the game when OOB
    if player.translation.y < -360.0 || player.translation.y > 360.0 {
        game_state.health_amount = 0;
    }
}

// ----------------------------------------------------------------------------------------------

fn move_road_objects(engine: &mut Engine) {
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
}

// ----------------------------------------------------------------------------------------------

fn handle_collisions(engine: &mut Engine, game_state: &mut GameState) {
    let health_message = engine.texts.get_mut("health_message").unwrap();

    // Go through all collision events and act accordingly
    for event in engine.collision_events.drain(..) {
        // We don't care if obstacles collide with each other or collisions end
        if !event.pair.either_contains(game_state.player_name) || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.7);
        }
    }
}

// ----------------------------------------------------------------------------------------------

fn check_health(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.75);
    }
}

// ----------------------------------------------------------------------------------------------

// The first parameter is always a `&mut Engine`, and the second parameter is a mutable reference to your custom game state struct (`&mut GameState` in this case).
// This function will be run once each frame.
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Don't run any more game logic if the game has ended
    if game_state.lost {
        return;
    }

    // Check for KB input
    handle_keyboard(engine, game_state);

    // Move road objects
    move_road_objects(engine);

    // Deal with collisions
    handle_collisions(engine, game_state);

    // End the game if out of car health
    check_health(engine, game_state);
}
