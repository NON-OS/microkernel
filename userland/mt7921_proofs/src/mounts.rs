// NONOS Operating System (AGPL-3.0-or-later)

//! The driver's modules, each mounted by path at the crate root, where the
//! driver's own `crate::` paths expect them. Only `main.rs` (the entry and
//! the attributes of a `no_std` binary) is left out.

#[path = "../../capsule_driver_mt7921/src/bringup/mod.rs"]
pub mod bringup;
#[path = "../../capsule_driver_mt7921/src/card.rs"]
pub mod card;
#[path = "../../capsule_driver_mt7921/src/clock.rs"]
pub mod clock;
#[path = "../../capsule_driver_mt7921/src/dma/mod.rs"]
pub mod dma;
#[path = "../../capsule_driver_mt7921/src/fw/mod.rs"]
pub mod fw;
#[path = "../../capsule_driver_mt7921/src/hw/mod.rs"]
pub mod hw;
#[path = "../../capsule_driver_mt7921/src/ids.rs"]
pub mod ids;
#[path = "../../capsule_driver_mt7921/src/init/mod.rs"]
pub mod init;
#[path = "../../capsule_driver_mt7921/src/mac/mod.rs"]
pub mod mac;
#[path = "../../capsule_driver_mt7921/src/mcu/mod.rs"]
pub mod mcu;
#[path = "../../capsule_driver_mt7921/src/power/mod.rs"]
pub mod power;
#[path = "../../capsule_driver_mt7921/src/radio/mod.rs"]
pub mod radio;
#[path = "../../capsule_driver_mt7921/src/say.rs"]
pub mod say;
#[path = "../../capsule_driver_mt7921/src/serve/mod.rs"]
pub mod serve;
#[path = "../../capsule_driver_mt7921/src/setup/mod.rs"]
pub mod setup;
#[path = "../../capsule_driver_mt7921/src/stage.rs"]
pub mod stage;
#[path = "../../capsule_driver_mt7921/src/vif/mod.rs"]
pub mod vif;
