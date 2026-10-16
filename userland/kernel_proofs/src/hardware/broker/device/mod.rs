// NONOS Operating System (AGPL-3.0-or-later)
//! The record and its BARs, without the bus and flag words this proof
//! does not read.

#[allow(dead_code)]
#[path = "../../../../../../src/hardware/broker/device/bar.rs"]
mod bar;
#[path = "../../../../../../src/hardware/broker/device/record.rs"]
mod record;

pub use bar::{Bar, BarKind};
pub use record::DeviceRecord;
