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

//! The legacy register window, granted by the broker. A failure leaves
//! the release to `sequence::run`, which takes every grant with the claim.

use nonos_libc::{mk_mmio_map, mk_pio_grant, MmioMapOut, PioGrantOut, BAR_KIND_MMIO, BAR_KIND_PIO};
use nonos_virtio::Mmio;

use crate::discover::Found;
use crate::regs::Regs;

const PAGE_MASK: u64 = 0xFFF;

pub fn grant(dev: Found, epoch: u64) -> Result<Regs, &'static str> {
    match dev.register_kind {
        BAR_KIND_MMIO => grant_mmio(dev, epoch),
        BAR_KIND_PIO => grant_pio(dev, epoch),
        _ => Err("virtio-blk: unsupported register bar"),
    }
}

fn grant_mmio(dev: Found, epoch: u64) -> Result<Regs, &'static str> {
    let refused = "virtio-blk: mmio map failed";
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let length = dev.register_size.checked_add(PAGE_MASK).ok_or(refused)? & !PAGE_MASK;
    let rc = mk_mmio_map(dev.device_id, epoch, dev.register_bar as u32, 0, 0, length, &mut out);
    let base = usize::try_from(out.user_va).map_err(|_| refused)?;
    let len = usize::try_from(out.length.min(length)).map_err(|_| refused)?;
    if rc < 0 || base == 0 {
        return Err(refused);
    }
    // SAFETY: MkMmioMap mapped `out.length` bytes at `user_va`, readable and
    // writable, until MkMmioUnmap or the claim's release; the view covers no
    // more than that, and no window is used after the release.
    Ok(Regs::mmio(unsafe { Mmio::new(base as *mut u8, len) }))
}

fn grant_pio(dev: Found, epoch: u64) -> Result<Regs, &'static str> {
    let mut out = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    if mk_pio_grant(dev.device_id, epoch, dev.register_bar, 0, &mut out) < 0 {
        return Err("virtio-blk: pio grant failed");
    }
    Ok(Regs::pio(out.grant_id))
}
