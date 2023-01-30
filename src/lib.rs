use bevy::prelude::{App, Plugin};

pub struct BoidsPlugin;
impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup() {
    println!("Hello, world!");
}
