// Pong from the tutorial from https://book.amethyst.rs/master/pong-tutorial/pong-tutorial-01.html

// The internal pong module contains the game data we'll simulate.
mod pong;

// This module contains our custom systems in the game.
// We could define them however we want, we're just choosing to keep them in a module named "systems". Under the hood, the system module is publically re-exporting modules defined deeper in its private namespace that contain the system implementations.
mod systems;

// Issue: Input Lag
// ---
// The input lag here is HORRIBLE. This is due to the default Render pipeline settings as well as the fact that the Input system needs to run before game logic within the same thread.
// TODO: Use a custom RenderGraph to reduce latency.
// TODO: Instead of using the default input bundle, determine how to add two systems on the same thread:
//  - thread_local InputSystem
//  - thread_local (user systems)
// See: https://github.com/amethyst/amethyst/issues/1801#issuecomment-586706604

pub fn main() -> amethyst::Result<()> {
  // Let's get logging set up first. The default logger will allow us to print info, warnings, and errors to the terminal window.
  amethyst::start_logger(Default::default());
  
  // Create a DisplayConfig with some icon data.
  let display_config = {
    use amethyst::window::{DisplayConfig, Icon};
    let mut config = DisplayConfig::default();

    let mut icon = Vec::new(); for _ in 0..(128*128) {
      icon.extend(vec![255, 0, 0, 255]);
    }
    config.loaded_icon = Some(Icon::from_rgba(icon, 128, 128).unwrap());
    config.title = String::from("Pong!");

    config
  };

  // Fundamentally Amethyst runs Systems to update the game's Data in a loop. This name is a little confusing, because we're not defining the game's *data*, we're actually defining the game's *Systems* which will UPDATE the data.
  // This might make more sense if the systems are defining their own data to update, which is likely because they need SOME data to do things even on an empty game data struct.
  // Not super sure what this data structure is.
  let game_data = {
    // We'll be setting up a set of renderer systems provided by Amethyst.
    use amethyst::renderer;

    // Set up an InputBundle with input bindings. For now we reference an input configuration file and import it as a string. This is really flimsy and difficult to extend, so I'll want to change this ASAP...
    let input_bundle = {
      use amethyst::utils::application_root_dir;
      use amethyst::input;

      let binding_path = application_root_dir()?.join("config").join("bindings.ron");
      input::InputBundle::<input::StringBindings>::new()
        .with_bindings_from_file(binding_path)?
    };

    amethyst::GameDataBuilder::default()
      // We want to add the Transform bundle to support Transform components.
      .with_bundle(amethyst::core::TransformBundle::new())?

      // We add a "Bundle" of multiple systems to set up a basic window and render loop quickly.
      // A Bundle is a collection of Systems.
      // This bundle, the RenderingBundle, dynamically accepts plugins that govern its behavior.
      .with_bundle(
        renderer::RenderingBundle::<renderer::types::DefaultBackend>::new()
          // RenderToWindow allows us to open a window and render to it.
          // This is what we pass our DisplayConfig to define window size, title, etc.
          .with_plugin(
            renderer::RenderToWindow::from_config(display_config)
              .with_clear([0, 0, 0, 1])
          )
          // RenderFlat2D will allow us to render entities with a SpriteRenderer component.
          .with_plugin(renderer::RenderFlat2D::default())
      )?

      // Add the InputBundle we created earlier.
      .with_bundle(input_bundle)?

      // Add the Paddle system.
      .with(systems::PaddleSystem, "paddle_system", &["input_system"])
  };
  
  // Construct the game and kick off the update loop by calling run().
  {
    use amethyst::utils::application_root_dir;
    use amethyst::Application;

    let mut game = Application::new(
      application_root_dir()?.join("assets"),
      pong::Pong,
      game_data
    )?;
    game.run();
  }

  Ok(())
}
