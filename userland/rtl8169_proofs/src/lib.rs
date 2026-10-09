// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The rtl8169 bring-up, proved without the card.
//!
//! The `#[path]` includes pull in the shipping driver source, so these tests
//! run the code that boots. The register type is built from a base address,
//! so a window in host memory stands in for BAR0 and a device modelled from
//! the datasheet answers from it. The broker, DMA mapping and server halves
//! talk to the kernel rather than to registers and stay out.
//!
//! The property that earns this crate its place is the station address. The
//! driver draws one and programs it; the factory address burned into the part
//! is the one identifier an amnesic machine would otherwise announce to every
//! network it joins, and the driver's promise is that it never falls back to
//! it. That is checked here with the entropy source switched off.

/// Which chip: the XID table and the per-version predicates.
#[path = "../../capsule_driver_rtl8169/src/chip/mod.rs"]
pub mod chip;
#[path = "../../capsule_driver_rtl8169/src/constants/mod.rs"]
pub mod constants;
/// The per-version steps: FIFO waits, the RXDV gate, quiescing.
#[path = "../../capsule_driver_rtl8169/src/hw/mod.rs"]
pub mod hw;
pub mod init;
/// PHYstatus decoded, and the line logged when the link changes.
#[path = "../../capsule_driver_rtl8169/src/link/mod.rs"]
pub mod link;
#[path = "../../capsule_driver_rtl8169/src/log/mod.rs"]
pub mod log;
/// The request header another capsule sends; decoded before anything else.
#[path = "../../capsule_driver_rtl8169/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_rtl8169/src/queue/mod.rs"]
pub mod queue;
/// The registers that moved on the RTL8125: doorbell, mask, status.
#[path = "../../capsule_driver_rtl8169/src/regmap/mod.rs"]
pub mod regmap;
#[path = "../../capsule_driver_rtl8169/src/regs.rs"]
pub mod regs;
/// The receive path: the one parser of what the part writes, a descriptor
/// whose length and flags come from the device.
#[path = "../../capsule_driver_rtl8169/src/rx/mod.rs"]
pub mod rx;
pub mod setup;
/// The send path, which rings the per-version doorbell.
#[path = "../../capsule_driver_rtl8169/src/tx/mod.rs"]
pub mod tx;

#[cfg(test)]
mod tests;
