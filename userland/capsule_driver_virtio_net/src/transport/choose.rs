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

//! Legacy or modern: modern whenever the device presents it.
//!
//! A transitional function carries both a legacy I/O window and the virtio
//! 1.0 structures. Only the modern transport can take ACCESS_PLATFORM,
//! which a device behind an IOMMU needs to send its DMA through it, so the
//! capabilities are always read and the legacy registers are used only by
//! a device that presents nothing else. Presenting the modern transport
//! means every structure its bring-up maps: common, notify and device
//! configuration. A transitional function missing one, or whose one cannot
//! be mapped, would fail every modern attempt, so it keeps its legacy
//! registers; a modern-only function has none to keep, so it is refused.
//! The rule is the shared transport's (`nonos_virtio::modern_caps`), which
//! the other virtio drivers follow with their own structures.

use nonos_virtio::{Bars, Broker, ModernCaps, Need, VirtioError};

/// The structures the modern bring-up maps besides common and notify.
const NEED: Need = Need { isr: false, device: true };

/// The capabilities to drive the function with when it presents a usable
/// common, notify and device configuration; `None` leaves it to the legacy
/// registers.
pub fn modern_caps(
    bars: &Bars,
    broker: &mut impl Broker,
) -> Result<Option<ModernCaps>, &'static str> {
    nonos_virtio::modern_caps(bars, broker, NEED).map_err(VirtioError::message)
}
