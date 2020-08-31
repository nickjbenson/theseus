// Define the PaddleSystem type. We want it to inherit from SystemDesc so that it receives a `build(world: &mut World)` function.
use amethyst::derive::SystemDesc;
use amethyst::ecs::SystemData;
#[derive(SystemDesc)]
pub struct PaddleSystem;

// Now we actually implement the System.
use amethyst::{core, ecs, input};
use crate::pong;
impl<'s> ecs::System<'s> for PaddleSystem {
  type SystemData = (
    // Our Paddle system:
    // - Mutates Transform data
    ecs::WriteStorage<'s, core::Transform>,
    // - Reads Paddle data (side, width, height)
    ecs::ReadStorage<'s, pong::Paddle>,
    // - Reads InputHandler (resource) data
    ecs::Read<'s, input::InputHandler<input::StringBindings>>
  );
  
  fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) { 
    use amethyst::ecs::{Join};
    // Here we use "join" provided by ecs::Join to join over the Paddle and Transform component storages. We could use "par_join" to iterate over this data in parallel, but since we only have two paddles in this case, that's more trouble than it's worth. 
    for (transform, paddle) in (&mut transforms, &paddles).join() {
      let movement = match paddle.side {
        pong::Side::Left => input.axis_value("left_paddle"),
        pong::Side::Right => input.axis_value("right_paddle")
      };

      if let Some(mv_amount) = movement {
        let scaled_amount = 1.2 * mv_amount as f32;
        transform.set_translation_y(
          (transform.translation().y + scaled_amount)
            // ".min()" and ".max()" are API sins.
            // Syntax like that is backwards and confusing to reason about.
            // TODO: Rewrite this as a single extension function ".clamp(min, max)".
            // Alternatively, "at_most()" and "at_least()" would be fine.
            // -Nick 2020-08-30
            .max(paddle.height * 0.5)
            .min(pong::ARENA_HEIGHT - paddle.height * 0.5)
        );
      }
    }
  }
}
