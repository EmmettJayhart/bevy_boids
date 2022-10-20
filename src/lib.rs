use bevy::prelude::{App, Plugin};

pub struct BoidPlugin;
impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup() {
    println!("Hello, world!");
}
