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

//! Legacy or modern, decided once per bring-up.
//!
//! A transitional function (legacy id, legacy I/O BAR present) keeps the
//! legacy path it has always used, without a single config-space read, so
//! a boot without an IOMMU drives it exactly as before. A function with a
//! modern-only id, or one with no I/O BAR to hold legacy registers, has its
//! capabilities read, and is driven modern when a usable common
//! configuration is among them. Without one the legacy path is left to try
//! (and to fail and give up the way it always has).

use crate::caps::ModernCaps;
use crate::pci::{has_io_bar, is_modern_id, Bars};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Legacy,
    Modern,
}

/// Whether the capabilities are worth reading at all.
pub fn wants_probe(device: u16, bars: &Bars) -> bool {
    is_modern_id(device) || !has_io_bar(bars)
}

/// The transport for a function whose capabilities parsed to `caps`.
pub fn choose(device: u16, bars: &Bars, caps: &ModernCaps) -> Kind {
    if wants_probe(device, bars) && caps.common.is_some() {
        Kind::Modern
    } else {
        Kind::Legacy
    }
}
