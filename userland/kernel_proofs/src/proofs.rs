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

//! The test groups, mounted from lib.rs and named at the crate root, with
//! what the proofs reach as `crate::...` though it lives in a group.

/// The broker's DMA flags, here so their `pub(super)` reaches every group.
#[path = "../../../src/hardware/broker/dma/flags.rs"]
pub mod dma_flags;

#[path = "proofs_1.rs"]
pub mod proofs_1;
#[path = "proofs_2.rs"]
pub mod proofs_2;
#[path = "proofs_3.rs"]
pub mod proofs_3;
#[path = "proofs_4.rs"]
pub mod proofs_4;
#[path = "proofs_5.rs"]
pub mod proofs_5;
#[path = "proofs_6.rs"]
pub mod proofs_6;
#[path = "proofs_7.rs"]
pub mod proofs_7;
#[path = "proofs_8.rs"]
pub mod proofs_8;

pub use proofs_1::amd_vi;
pub use proofs_2::{iommu_window, mmio_window};
pub(crate) use proofs_3::authorization_tests;
pub use proofs_4::{lifecycle_state, usb_msc_report_words, watch_rule};
pub use proofs_5::{arm_acpi, arm_acpi_tests, seeded_tables, smbios};
pub(crate) use proofs_6::inventory_ids;
pub use proofs_8::legacy_ports;
