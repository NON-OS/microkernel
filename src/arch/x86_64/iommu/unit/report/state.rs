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

//! What the probe found, held for the bring-up that runs after it.

use spin::Once;

use super::super::probe::{UnitInfo, MAX_UNITS};
use crate::arch::x86_64::iommu::regs::cap;

struct Probed {
    units: heapless::Vec<UnitInfo, MAX_UNITS>,
    shared: UnitInfo,
}

static PROBED: Once<Probed> = Once::new();

/// What the remapping units support together, or `None` if the probe never
/// reached them. The register handle is the first unit's; anything that talks
/// to hardware walks `units()` instead. Bring-up treats `None` as "there is
/// nothing to program".
pub fn probed() -> Option<&'static UnitInfo> {
    PROBED.get().map(|p| &p.shared)
}

/// Every unit the probe reached, in DMAR order. Empty before the probe.
pub fn units() -> &'static [UnitInfo] {
    PROBED.get().map(|p| p.units.as_slice()).unwrap_or(&[])
}

/// Why a set of units cannot share one set of tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MergeError {
    Empty,
    NoCommonDepth,
}

/// Fold the units into what they support together. Feature bits survive only
/// when every unit sets them, widths and counts take the smallest, and the
/// cautious behaviours (caching mode, write buffer flushing, firmware-enabled
/// translation) hold when any unit reports them.
pub(super) fn merge(units: &[UnitInfo]) -> Result<UnitInfo, MergeError> {
    let first = units.first().ok_or(MergeError::Empty)?;
    let mut caps = [0u64; MAX_UNITS];
    let mut ecaps = [0u64; MAX_UNITS];
    for (i, info) in units.iter().take(MAX_UNITS).enumerate() {
        caps[i] = info.cap;
        ecaps[i] = info.ecap;
    }
    let n = units.len().min(MAX_UNITS);
    let levels = cap::shared_levels(&caps[..n]).ok_or(MergeError::NoCommonDepth)?;
    Ok(UnitInfo {
        unit: first.unit,
        version: first.version,
        cap: cap::all_support(&caps[..n]),
        ecap: cap::all_support(&ecaps[..n]),
        levels,
        max_address_width: cap::shared_address_width(&caps[..n]),
        domains: cap::shared_domain_count(&caps[..n]),
        caching_mode: units.iter().any(|u| u.caching_mode),
        requires_write_buffer_flush: units.iter().any(|u| u.requires_write_buffer_flush),
        translation_enabled: units.iter().any(|u| u.translation_enabled),
    })
}

/// Latch the probe result. Called once, from `init`, and only on the path
/// where every unit answered and they agree on a depth.
pub(super) fn record(units: heapless::Vec<UnitInfo, MAX_UNITS>, shared: UnitInfo) {
    PROBED.call_once(|| Probed { units, shared });
}
