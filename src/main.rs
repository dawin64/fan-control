use clap::Parser;
use data::{
    cli::Args, config::Config, directories::DirManager, node::AppGraph, serde_helper,
    update::Update, AppState,
};
use hardware::{self, HardwareBridge};

use ui::run_ui;

#[macro_use]
extern crate log;

#[cfg(all(test, feature = "fake_hardware"))]
mod integrated_test;

fn main() {
    env_logger::init();
    ui::localize::localize();
    data::localize::localize();

    let args = Args::parse();

    let dir_manager = DirManager::new(args);

    #[cfg(feature = "fake_hardware")]
    let (hardware, bridge) = hardware::fake_hardware::FakeHardwareBridge::generate_hardware();

    #[cfg(all(not(feature = "fake_hardware"), target_os = "linux"))]
    let (hardware, bridge) = hardware::linux::LinuxBridge::generate_hardware();

    #[cfg(all(not(feature = "fake_hardware"), target_os = "windows"))]
    let (hardware, bridge) = hardware::windows::WindowsBridge::generate_hardware();

    let hardware_file_path = dir_manager.hardware_file_path();

    if let Err(e) = serde_helper::serialize(&hardware_file_path, &hardware) {
        warn!("{}", e);
    }

    let config = match &dir_manager.settings.current_config {
        Some(config_name) => {
            match serde_helper::deserialize::<Config>(&dir_manager.config_file_path(config_name)) {
                Ok(config) => Some(config),
                Err(e) => {
                    warn!("{:?}", e);
                    None
                }
            }
        }
        None => None,
    };

    let app_graph = match config {
        Some(config) => AppGraph::from_config(config, &hardware),
        None => AppGraph::default(&hardware),
    };

    let app_state = AppState {
        dir_manager,
        hardware,
        bridge,
        app_graph,
        update: Update::new(),
    };

    run_ui(app_state).unwrap();
}
