// Deps
use amethyst::SimpleState;

// Game state struct.
pub struct Pong;

// Implement SimpleState, which adds type info to the game state that tells Amethyst what to do when the close signal is received from the OS.
//
// See: https://docs.amethyst.rs/stable/src/amethyst/state.rs.html#265
//
// We can then additionally implement whatever basic callbacks we want to have for the lifecycle the game state, such as on_start.
use amethyst::StateData;
use amethyst::GameData;
impl SimpleState for Pong {

  // This will allow us to implement initialization for the Pong game data.
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    // On init, we'll create the entity with the Camera and Transform and add it to the world (the resource container that contains things like entities with components).
    initialize_camera(world);

    // And we'll initialize the paddles.
    // Initialize the spritesheet for paddle graphics.
    let spritesheet_handle = load_spritesheet(world);

    // Because there's no System that uses the Paddle component yet, the world isn't able to handle storing the component's data. To fix this for now, we'll just manually register the component's with the world.
    // use amethyst::prelude::WorldExt;
    // world.register::<Paddle>();
    // (Now there's a PaddleSystem, so no need to call world.register any more.)
    initialize_paddles(world, spritesheet_handle);
  }

}

// Public constants for the arena that we'll when setting up the camera and in other modules.
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

fn initialize_camera(world: &mut amethyst::prelude::World) {
  // We'll initialize a 2D camera that with the specified arena's width and height.
  use amethyst::renderer;
  let _camera = renderer::Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT);

  // The camera's Transform will be placed at a standard Z depth back -- not that this matters, because the projection is orthographic. (Theoretically it only matters for the clipping plane.)
  use amethyst::core;
  let mut transform = core::Transform::default();
  transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
  
  // use amethyst::prelude::WorldExt;
  use amethyst::prelude::Builder;
  world
    .create_entity()
    .with(renderer::Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
    .with(transform)
    .build();
}

// Paddle Component //

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

#[derive(PartialEq, Eq)]
pub enum Side {
  Left,
  Right
}

pub struct Paddle {
  pub side: Side,
  pub width: f32,
  pub height: f32
}

impl Paddle {
  fn new(side: Side) -> Self {
    Paddle {
      side,
      width: PADDLE_WIDTH,
      height: PADDLE_HEIGHT
    }
  }
}

// This is the part where the Paddle becomes a Component. The Component trait defines for specs how its data should be organized (this is just a matter of optimization -- depending on the type of data and how it's used, different storage is ideal).
use amethyst::ecs;
impl ecs::Component for Paddle {
  type Storage = ecs::DenseVecStorage<Self>;
}

use amethyst::core::Transform;
use amethyst::renderer::SpriteRender;

/// Initializes a paddle on the left and a paddle on the right.
fn initialize_paddles(world: &mut amethyst::prelude::World, spritesheet_handle: Handle<SpriteSheet>) {
  let mut left_xfm = Transform::default();
  let mut right_xfm = Transform::default();

  // Position the paddle transforms.
  let y = ARENA_HEIGHT / 2.0;
  left_xfm.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
  right_xfm.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

  // We'll also add a SpriteRender component.
  let sprite_render = SpriteRender::new(spritesheet_handle, 0);

  // Create the entities.
  use amethyst::prelude::Builder;
  // use amethyst::prelude::WorldExt;
  world
    .create_entity()
    .with(Paddle::new(Side::Left))
    .with(left_xfm)
    .with(sprite_render.clone())
    .build();
  world
    .create_entity()
    .with(Paddle::new(Side::Right))
    .with(right_xfm)
    .with(sprite_render)
    .build();
}

// Paddle Sprite(sheet) //

use amethyst::ecs::World;
use amethyst::ecs::WorldExt;
use amethyst::assets::Handle;
use amethyst::assets::Loader;
use amethyst::assets::AssetStorage;
use amethyst::renderer::Texture;
use amethyst::renderer::SpriteSheet;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::renderer::ImageFormat;

fn load_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
  // Load texture data from the PNG spritesheet for the paddle.
  let texture_handle = {
    // The asset loader is a Resource. Resources are types of data  It's stored in the World and created when the Application is created. It handles manages the loading of all kinds of Assets.
    let loader = world.read_resource::<Loader>();

    // Once the Loader creates the Texture asset, it needs to be told where to store that asset data.
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();

    // Here, we use the Loader to load a PNG file into a Texture asset. The returned value is a Handle; this provides access to where the Texture **will** be once it is loaded (it doesn't load immediately.)
    loader.load(
      "texture/pong_spritesheet.png",
      ImageFormat::default(),
      (),
      &texture_storage
    )
  };

  // Now, load the paddle sprite sheet using the texture and a SpriteSheetFormat definition for the asset.
  {
    let loader = world.read_resource::<Loader>();
    let spritesheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
      "texture/pong_spritesheet.ron", // Encodes where the sprites are in the spritesheet
      SpriteSheetFormat(texture_handle),
      (),
      &spritesheet_store
    )
  }
}
