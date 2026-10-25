// NONOS Operating System (AGPL-3.0-or-later)
//! F-128: what stops a DMA master the broker found through ACPI before its
//! frames are freed. The rules are included by path. The model, stated
//! apart from them:
//! - of the ACPI kinds the platform rule records (an I2C host as SERIAL,
//!   an SD/eMMC host as BLOCK, a GPIO controller, PS/2 as INPUT, I2C-HID,
//!   the SCI as OTHER), only the SD host masters the bus; every other
//!   ACPI kind, and any class not known, gets no DMA; a device found any
//!   other way keeps its own rule;
//! - an SD host is stopped by writing Software Reset For All (bit 0) to
//!   the register at 0x2F (SD Host Controller Simplified Specification, as
//!   recalled, unverified) and reading it back until bit 0 is clear;
//! - its frames are freed only when that read-back was seen; a host still
//!   resetting at 100 ms, or unreachable, keeps them, and the wait ends
//!   even on a clock that never moves.

#[path = "../../../../../src/hardware/broker/dma/acpi_stop.rs"]
pub mod acpi_stop;
#[path = "../../../../../src/hardware/broker/dma/sdhci_stop.rs"]
pub mod sdhci_stop;

mod host;
mod reset_tests;
mod table_tests;
