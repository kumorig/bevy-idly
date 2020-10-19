extern crate chrono;
extern crate timer;
mod init;
mod costs;
mod resources;

use bevy::prelude::App;
use bevy::prelude::*;
use init::ResPlugin;


fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 300,
            height: 600,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(ResPlugin)
        .run();
}
