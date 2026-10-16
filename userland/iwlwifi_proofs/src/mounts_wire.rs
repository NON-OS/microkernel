// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's request wire, its control family and its join messages.

#[path = "../../capsule_driver_iwlwifi/src/server/control/mod.rs"]
pub mod control;
/// The control family's join messages as the driver reads and answers them.
#[path = "../../capsule_driver_iwlwifi/src/server/join_wire/mod.rs"]
pub mod join_wire;
/// The Wi-Fi client's table of drivers, held to what this driver answers.
#[path = "../../nonos_wifi_client/src/driver/services.rs"]
pub mod wifi_services;
