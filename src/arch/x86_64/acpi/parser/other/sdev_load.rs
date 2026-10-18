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

//! SDEV read once at boot through the one parser (`shared_tables::sdev`)
//! and kept, read-only, for the broker's posture lines. What the IOMMU
//! does about a listed device is not decided here.

use super::super::state::TableRegistry;
use super::raw_table::bytes;
use super::shared_tables::sdev::{sdev, SdevEntry};
use alloc::vec::Vec;
use spin::RwLock;

static SDEV: RwLock<Vec<SdevEntry>> = RwLock::new(Vec::new());

pub fn parse_sdev(registry: &mut TableRegistry) {
    *SDEV.write() = bytes(registry, b"SDEV").and_then(|t| sdev(t).ok()).unwrap_or_default();
}

/// The devices firmware asks to keep secured; empty without an SDEV.
pub fn sdev_entries() -> Vec<SdevEntry> {
    SDEV.read().clone()
}
