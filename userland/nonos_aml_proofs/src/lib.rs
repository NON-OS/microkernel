// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the AML interpreter. The modules below are the shipping
//! sources of `userland/nonos_aml`, at the `crate::` paths they name each
//! other by.

extern crate alloc;

#[path = "../../nonos_aml/src/aei/mod.rs"]
pub mod aei;
#[path = "../../nonos_aml/src/apei/mod.rs"]
pub mod apei;
#[path = "../../nonos_aml/src/arena/mod.rs"]
pub mod arena;
#[path = "../../nonos_aml/src/block.rs"]
pub mod block;
#[path = "../../nonos_aml/src/button/mod.rs"]
pub mod button;
#[path = "../../nonos_aml/src/cpu/mod.rs"]
pub mod cpu;
#[path = "../../nonos_aml/src/device_power/mod.rs"]
pub mod device_power;
#[path = "../../nonos_aml/src/ec/mod.rs"]
pub mod ec;
#[path = "../../nonos_aml/src/error.rs"]
pub mod error;
#[path = "../../nonos_aml/src/events/mod.rs"]
pub mod events;
#[path = "../../nonos_aml/src/exec/mod.rs"]
pub mod exec;
#[path = "../../nonos_aml/src/fan/mod.rs"]
pub mod fan;
#[path = "../../nonos_aml/src/ged/mod.rs"]
pub mod ged;
#[path = "../../nonos_aml/src/grammar/mod.rs"]
pub mod grammar;
#[path = "../../nonos_aml/src/legacy/mod.rs"]
pub mod legacy;
#[path = "../../nonos_aml/src/motherboard/mod.rs"]
pub mod motherboard;
#[path = "../../nonos_aml/src/load/mod.rs"]
pub mod load;
#[path = "../../nonos_aml/src/namespace/mod.rs"]
pub mod namespace;
#[path = "../../nonos_aml/src/object/mod.rs"]
pub mod object;
#[path = "../../nonos_aml/src/op/mod.rs"]
pub mod op;
#[path = "../../nonos_aml/src/parse/mod.rs"]
pub mod parse;
#[path = "../../nonos_aml/src/platform/mod.rs"]
pub mod platform;
#[path = "../../nonos_aml/src/platform_device/mod.rs"]
pub mod platform_device;
#[path = "../../nonos_aml/src/ports/mod.rs"]
pub mod ports;
#[path = "../../nonos_aml/src/props/mod.rs"]
pub mod props;
#[path = "../../nonos_aml/src/power/mod.rs"]
pub mod power;
#[path = "../../nonos_aml/src/region/mod.rs"]
pub mod region;
#[path = "../../nonos_aml/src/serial/mod.rs"]
pub mod serial;
#[path = "../../nonos_aml/src/shape/mod.rs"]
pub mod shape;
#[path = "../../nonos_aml/src/sleep/mod.rs"]
pub mod sleep;
#[path = "../../nonos_aml/src/suspend/mod.rs"]
pub mod suspend;
#[path = "../../nonos_aml/src/thermal/mod.rs"]
pub mod thermal;
#[path = "../../nonos_aml/src/time_alarm/mod.rs"]
pub mod time_alarm;
#[path = "../../nonos_aml/src/video/mod.rs"]
pub mod video;
#[path = "../../nonos_aml/src/wmi/mod.rs"]
pub mod wmi;

#[cfg(test)]
mod support;
#[cfg(test)]
use support::*;
#[cfg(test)]
mod tests;
