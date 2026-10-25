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

//! The ACPI tables an Arm machine describes itself with, parsed from the
//! bytes alone: MADT's GIC structures, GTDT, SPCR, PPTT, IORT, SRAT,
//! SLIT, HMAT, DBG2, PCCT and SDEV. Pure, and proved in
//! `kernel_proofs`; nothing calls them until the UEFI boot path hands the
//! kernel these tables.

pub mod dbg2;
mod dbg2_device;
pub mod gtdt;
pub mod iort;
mod le;
pub mod madt;
pub mod numa;
pub mod pcct;
mod pcct_extended;
mod pcct_subspace;
mod pcct_types;
pub mod pptt;
pub mod sdev;
mod sdev_entry;
pub mod sdt;
pub mod spcr;
mod spcr_fields;
mod spcr_tail;
