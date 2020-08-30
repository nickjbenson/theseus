// Deps
use amethyst::SimpleState;

// Game state struct.
pub struct Pong;

// Implement SimpleState, which adds type info to the game state that tells Amethyst what to do when the close signal is received from the OS.
// See: https://docs.amethyst.rs/stable/src/amethyst/state.rs.html#265
use amethyst::StateData;
use amethyst::GameData;
impl SimpleState for Pong {
  // This will allow us to implement initialization for the Pong game data.
  fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    // Initialization here.
  }
}

// fn initialize_camera(world: &mut World) {

// }

// // Public constants for the arena that we'll use in other modules.
// pub const ARENA_HEIGHT: f32 = 100.0;
// pub const ARENA_WIDTH: f32 = 100.0;
