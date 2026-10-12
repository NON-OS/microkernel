// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's request wire, its control family and its join messages.

#[path = "../../capsule_driver_iwlwifi/src/protocol/ops.rs"]
mod protocol_ops;
/// The protocol op numbers the serving loop's guard names.
pub mod protocol {
    pub use super::protocol_ops::*;
}
#[path = "../../capsule_driver_iwlwifi/src/server/control.rs"]
pub mod control;
#[path = "../../capsule_driver_iwlwifi/src/server/guard.rs"]
pub mod guard;
/// The whole request wire the serving loop reads with: the header decode
/// every frame passes before dispatch, and the reply encoder a refusal is
/// answered with. Its ops file is also `protocol_ops`, loaded twice on purpose.
#[allow(clippy::duplicate_mod)]
#[path = "../../capsule_driver_iwlwifi/src/protocol/mod.rs"]
pub mod iwlwifi_wire;
/// The control family's join messages as the driver reads and answers them.
#[path = "../../capsule_driver_iwlwifi/src/server/join_wire.rs"]
pub mod join_wire;
/// The Wi-Fi client's table of drivers, held to what this driver answers.
#[path = "../../nonos_wifi_client/src/driver/services.rs"]
pub mod wifi_services;
