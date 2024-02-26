use rand::prelude::*;
use rusty_engine::prelude::*;

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

/// Represents the state of the game.
///
/// This struct holds information about the player's name, health amount, and whether the game
/// is lost or not.
#[derive(Resource)]
struct GameState<'name> {
    player_name: &'name str,
    health_amount: u8,
    lost: bool,
}

// ----------------------------------------------------------------------------------------------
// Main helpers
// ----------------------------------------------------------------------------------------------
/// Adds a player to the game with the specified name and car sprite.
///
/// This function creates a new player sprite with the given `player_name` and `car` sprite preset,
/// adds it to the `game`, sets its initial position and collision properties.
///
/// # Arguments
///
/// * `game` - A mutable reference to the game instance where the player will be added.
/// * `player_name` - The name of the player.
/// * `car` - The sprite preset representing the player's car.
///
/// # Example
///
/// ```
/// use my_game_library::{Game, GameState, SpritePreset};
///
/// let mut game = Game::<GameState>::new();
/// add_player(&mut game, "Player 1", SpritePreset::Car);
/// ```
fn add_player<'game>(game: &'game mut Game<GameState>, player_name: &'game str, car: SpritePreset) {
    let player = game.add_sprite(player_name, car);
    player.translation.x = -500.0;
    player.layer = 10.0;
    player.collision = true;
}

// ----------------------------------------------------------------------------------------------
/// Sets the game audio to play the specified music preset at the given volume.
///
/// This function plays the specified `music` preset with the provided `volume` using the game's
/// audio manager.
///
/// # Arguments
///
/// * `game` - A mutable reference to the game instance where the audio will be played.
/// * `music` - The preset representing the music to be played.
/// * `volume` - The volume level at which the music will be played, ranging from 0.0 (silent) to 1.0 (full volume).
///
/// # Example
///
/// ```
/// use my_game_library::{Game, GameState, MusicPreset};
///
/// let mut game = Game::<GameState>::new();
/// set_game_audio(&mut game, MusicPreset::Background, 0.8);
/// ```
fn set_game_audio(game: &mut Game<GameState>, music: MusicPreset, volume: f32) {
    game.audio_manager.play_music(music, volume);
}

// ----------------------------------------------------------------------------------------------
/// Creates road lines in the game with the specified barrier sprite.
///
/// This function generates a series of road lines with the given `barrier` sprite preset
/// and adds them to the `game`.
///
/// # Arguments
///
/// * `game` - A mutable reference to the game instance where the road lines will be created.
/// * `barrier` - The sprite preset representing the barrier used for road lines.
///
/// # Example
///
/// ```
/// use my_game_library::{Game, GameState, SpritePreset};
///
/// let mut game = Game::<GameState>::new();
/// create_road_lines(&mut game, SpritePreset::Barrier);
/// ```
fn create_road_lines(game: &mut Game<GameState>, barrier: SpritePreset) {
    const NUM_LINES: u8 = 10;

    for i in 0..NUM_LINES {
        let roadline = game.add_sprite(format!("roadline{}", i), barrier);
        roadline.scale = 0.2;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }
}

// ----------------------------------------------------------------------------------------------
/// Adds obstacles to the game.
///
/// This function initializes and adds obstacles to the game instance. The obstacles are created
/// with predefined presets and randomized positions within the game world.
///
/// # Arguments
///
/// * `game` - A mutable reference to the game instance where the obstacles will be added.
///
/// # Example
///
/// ```
/// use my_game_library::{Game, GameState};
///
/// let mut game = Game::<GameState>::new();
/// add_obstacles(&mut game);
/// ```
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
/// Creates a message in the game with the specified label, text, and position.
///
/// This function creates a message with the given `label` and `text` content and adds it to
/// the `game` instance at the specified position (`x`, `y`).
///
/// # Arguments
///
/// * `game` - A mutable reference to the game instance where the message will be created.
/// * `label` - The label or identifier for the message.
/// * `text` - The content text of the message.
/// * `x` - The x-coordinate position where the message will be placed.
/// * `y` - The y-coordinate position where the message will be placed.
///
/// # Example
///
/// ```
/// use my_game_library::{Game, GameState};
///
/// let mut game = Game::<GameState>::new();
/// create_message(&mut game, "Info", "Welcome to the game!", 100.0, 200.0);
/// ```
fn create_message(game: &mut Game<GameState>, label: &str, text: &str, x: f32, y: f32) {
    let message = game.add_text(label, text);
    message.translation = Vec2::new(x, y);
}

// ----------------------------------------------------------------------------------------------
/// Entry point of the game.
///
/// This function initializes the game, creates the player sprite, sets up audio and visual elements,
/// and runs the game logic.
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
/// Handles keyboard input for player movement and game control.
///
/// This function updates the player's position and rotation based on keyboard input,
/// as well as checks for out-of-bounds conditions to end the game.
///
/// # Arguments
///
/// * `engine` - A mutable reference to the game engine.
/// * `game_state` - A mutable reference to the current game state.
///
/// # Example
///
/// ```
/// use my_game_library::{Engine, GameState};
///
/// fn main() {
///     let mut engine = Engine::new();
///     let mut game_state = GameState::new();
///
///     handle_keyboard(&mut engine, &mut game_state);
/// }
/// ```
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
/// Moves road objects (road lines and obstacles) horizontally across the screen.
///
/// This function updates the position of road lines and obstacles by moving them to the left,
/// simulating the effect of the player's movement. If road objects move out of the screen,
/// they are repositioned to the other side to create an endless scrolling effect.
///
/// # Arguments
///
/// * `engine` - A mutable reference to the game engine.
///
/// # Example
///
/// ```
/// use my_game_library::Engine;
///
/// fn main() {
///     let mut engine = Engine::new();
///     move_road_objects(&mut engine);
/// }
/// ```
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
/// Handles collision events between game objects.
///
/// This function processes collision events between game objects, specifically between the player
/// and obstacles. It reduces the player's health upon collision, updates the health message, and
/// plays a sound effect.
///
/// # Arguments
///
/// * `engine` - A mutable reference to the game engine.
/// * `game_state` - A mutable reference to the current game state.
///
/// # Example
///
/// ```
/// use my_game_library::{Engine, GameState};
///
/// fn main() {
///     let mut engine = Engine::new();
///     let mut game_state = GameState::new();
///
///     handle_collisions(&mut engine, &mut game_state);
/// }
/// ```
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
/// Checks the player's health status and handles game over conditions.
///
/// This function checks the player's health amount, and if it reaches zero, it sets the game
/// state to lost, displays a "Game Over" message, stops the game music, and plays a game over sound effect.
///
/// # Arguments
///
/// * `engine` - A mutable reference to the game engine.
/// * `game_state` - A mutable reference to the current game state.
///
/// # Example
///
/// ```
/// use my_game_library::{Engine, GameState};
///
/// fn main() {
///     let mut engine = Engine::new();
///     let mut game_state = GameState::new();
///
///     check_health(&mut engine, &mut game_state);
/// }
/// ```
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
/// Handles the game logic, including player input, object movement, collisions, and game over conditions.
///
/// This function orchestrates various aspects of the game, including handling player input,
/// moving road objects, detecting collisions, and checking for game over conditions such as
/// running out of health.
/// The first parameter is always a `&mut Engine`, and the second parameter is a mutable reference to your custom game state struct (`&mut GameState` in this case).
/// This function will be run once each frame.
///
/// # Arguments
///
/// * `engine` - A mutable reference to the game engine.
/// * `game_state` - A mutable reference to the current game state.
///
/// # Example
///
/// ```
/// use my_game_library::{Engine, GameState};
///
/// fn main() {
///     let mut engine = Engine::new();
///     let mut game_state = GameState::new();
///
///     game_logic(&mut engine, &mut game_state);
/// }
/// ```
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
