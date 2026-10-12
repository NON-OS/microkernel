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
//! a device that presents nothing else.

use nonos_virtio::{parse, Bars, Broker, ConfigSpace, ModernCaps, VirtioError};

/// The capabilities to drive the function with when it presents a usable
/// common configuration; `None` leaves it to the legacy registers.
pub fn modern_caps(
    bars: &Bars,
    broker: &mut impl Broker,
) -> Result<Option<ModernCaps>, &'static str> {
    let cfg = ConfigSpace::read(broker).ok_or(VirtioError::ConfigRead.message())?;
    let caps = parse(&cfg, bars);
    Ok(caps.common.is_some().then_some(caps))
}
