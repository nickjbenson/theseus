// "pub use" means we're re-exporting the PaddleSystem from self::paddle::PaddleSystem to this systems module. So a consumer of this module would see PaddleSystem as "systems::PaddleSystem", even though within this module, it's nested differently.
pub use self::paddle::PaddleSystem;

// We declare that this module has a nested module that can be found in paddle.rs.
mod paddle;
