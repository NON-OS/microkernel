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

//! The MMIO mappings an attempt makes: BAR0 uncached (registers must not
//! be combined or reordered), and a window of the aperture write-combining
//! for the one read of firmware's picture.

use nonos_libc::{mk_mmio_map, MmioMapOut};

use crate::error::{IntelError, IntelResult};
use crate::handles::BrokerHandles;

/// `MK_MMIO_MAP_WC` (L17): only on a prefetchable BAR, which GMADR is.
const MK_MMIO_MAP_WC: u32 = 1 << 0;

pub fn bar0(h: &mut BrokerHandles, len: u64) -> IntelResult<MmioMapOut> {
    let out = mmio(h, 0, 0, 0, len)?;
    h.bar0 = Some(out.grant_id);
    Ok(out)
}

/// `len` bytes of the aperture at `offset`; the caller unmaps it.
pub fn aperture(h: &BrokerHandles, offset: u64, len: u64) -> IntelResult<MmioMapOut> {
    mmio(h, 2, MK_MMIO_MAP_WC, offset, len)
}

fn mmio(h: &BrokerHandles, bar: u32, flags: u32, off: u64, len: u64) -> IntelResult<MmioMapOut> {
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let rc = mk_mmio_map(h.device_id, h.epoch, bar, flags, off, len, &mut out);
    if rc < 0 {
        return Err(IntelError::Broker(rc));
    }
    Ok(out)
}
