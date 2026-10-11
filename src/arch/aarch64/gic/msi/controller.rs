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

use core::sync::atomic::{AtomicU64, AtomicU8, Ordering};

use spin::Once;

use super::super::state::version;
use super::super::version::GicVersion;
use super::super::{its, v2m};
use crate::arch::fdt::find::msi::{MsiController, MsiKind};
use crate::arch::fdt::find::smmu::IommuMap;

pub(super) const NONE: u8 = 0;
pub(super) const ITS: u8 = 1;
pub(super) const V2M: u8 = 2;

static KIND: AtomicU8 = AtomicU8::new(NONE);
static DOORBELL: AtomicU64 = AtomicU64::new(0);
static MAP: Once<(u32, IommuMap)> = Once::new();

/// Bring up the controller the host bridge's map names, and keep the map:
/// it is what turns a function's requester id into its id there. Returns
/// what the boot log calls it.
pub fn bring_up(ctrl: &MsiController, map: &IommuMap) -> Result<&'static str, &'static str> {
    let (kind, doorbell, name) = match ctrl.kind {
        MsiKind::Its if version() != Some(GicVersion::V3) => {
            return Err("the tree names an ITS beside a GICv2");
        }
        MsiKind::Its => (ITS, its::bring_up(ctrl.base, device_ids(map))?, "gic-v3-its"),
        MsiKind::V2m { spi_base, spi_count } => {
            (V2M, v2m::bring_up(ctrl.base, spi_base, spi_count)?, "gic-v2m")
        }
    };
    MAP.call_once(|| (ctrl.phandle, *map));
    DOORBELL.store(doorbell, Ordering::Release);
    KIND.store(kind, Ordering::Release);
    Ok(name)
}

/// The controller's doorbell, `None` before one is in service.
pub fn doorbell() -> Option<u64> {
    (kind() != NONE).then(|| DOORBELL.load(Ordering::Acquire))
}

/// The id a function with requester id `rid` writes under, `None` when the
/// map sends it to no controller this kernel drives.
pub fn device_id(rid: u16) -> Option<u32> {
    let (phandle, map) = MAP.get()?;
    map.output(rid, *phandle)
}

pub(super) fn kind() -> u8 {
    KIND.load(Ordering::Acquire)
}

/// One past the highest id the map hands out, which sizes the ITS's
/// device table.
fn device_ids(map: &IommuMap) -> u64 {
    let ends = map.entries[..map.count].iter().map(|e| u64::from(e.sid_base) + u64::from(e.length));
    ends.max().unwrap_or(0)
}
