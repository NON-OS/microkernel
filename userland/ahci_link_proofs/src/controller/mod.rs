// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../capsule_driver_ahci/src/controller/enable.rs"]
mod enable;
#[path = "../../../capsule_driver_ahci/src/controller/handoff.rs"]
mod handoff;
#[path = "../../../capsule_driver_ahci/src/controller/info.rs"]
mod info;
#[path = "../../../capsule_driver_ahci/src/controller/port_info.rs"]
mod port_info;
#[path = "../../../capsule_driver_ahci/src/controller/ports.rs"]
pub mod ports;
#[path = "../../../capsule_driver_ahci/src/controller/remap.rs"]
pub mod remap;
#[path = "../../../capsule_driver_ahci/src/controller/restore.rs"]
pub mod restore;
#[path = "../../../capsule_driver_ahci/src/controller/scan_ports.rs"]
mod scan_ports;
#[path = "../../../capsule_driver_ahci/src/controller/settle.rs"]
mod settle;
#[path = "../../../capsule_driver_ahci/src/controller/signature.rs"]
mod signature;
#[path = "../../../capsule_driver_ahci/src/controller/spin_up.rs"]
mod spin_up;
#[path = "../../../capsule_driver_ahci/src/controller/stopped.rs"]
mod stopped;
mod stopped_tests;
#[path = "../../../capsule_driver_ahci/src/controller/window.rs"]
pub mod window;

pub use enable::enable_ahci;
pub use info::ControllerInfo;
pub use port_info::PortInfo;
pub use scan_ports::scan_ports;
pub use settle::settle_links;
pub use spin_up::spin_up;
pub use stopped::{armed_port, stop_armed};
pub use window::ports_in_window;
