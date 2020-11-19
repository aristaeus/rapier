use crate::testbed::PhysicsState;
use kiss3d::window::Window;
use na::Point3;

pub trait TestbedPlugin {
    fn init_graphics(&mut self, window: &mut Window, gen_color: &mut dyn FnMut() -> Point3<f32>);
    fn clear_graphics(&mut self, window: &mut Window);
    fn run_callbacks(&mut self, window: &mut Window, physics: &mut PhysicsState, t: f32);
    fn step(&mut self, physics: &mut PhysicsState);
    fn draw(&mut self);
    fn profiling_string(&self) -> String;
}
