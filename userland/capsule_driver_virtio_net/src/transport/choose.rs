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

use nonos_virtio::pci::is_modern_id;
use nonos_virtio::{parse, Bars, Broker, ConfigSpace, ModernCaps, VirtioError};

/// The capabilities to drive the function with when it presents a usable
/// common, notify and device configuration; `None` leaves it to the legacy
/// registers.
pub fn modern_caps(
    bars: &Bars,
    broker: &mut impl Broker,
) -> Result<Option<ModernCaps>, &'static str> {
    let cfg = ConfigSpace::read(broker).ok_or(VirtioError::ConfigRead.message())?;
    let caps = parse(&cfg, bars);
    let usable = caps.common.is_some() && caps.notify.is_some() && caps.device.is_some();
    if !usable && cfg.u16_at(2).is_some_and(is_modern_id) {
        return Err("virtio-net: modern-only function without its modern structures");
    }
    Ok(usable.then_some(caps))
}
