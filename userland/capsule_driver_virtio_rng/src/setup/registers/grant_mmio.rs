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

use crate::discover::Found;
use crate::regs::Regs;
use nonos_libc::{mk_mmio_map, MmioMapOut};
use nonos_virtio::Mmio;

const PAGE_MASK: u64 = 0xFFF;

pub(super) fn grant_mmio(dev: Found, epoch: u64) -> Result<Regs, &'static str> {
    let length = dev.register_size.checked_add(PAGE_MASK).ok_or("mmio map failed")? & !PAGE_MASK;
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let r = mk_mmio_map(dev.device_id, epoch, dev.register_bar as u32, 0, 0, length, &mut out);
    let base = usize::try_from(out.user_va).map_err(|_| "mmio map failed")?;
    let len = usize::try_from(out.length).map_err(|_| "mmio map failed")?;
    if r < 0 || base == 0 {
        return Err("mmio map failed");
    }
    // SAFETY: MkMmioMap mapped `out.length` bytes at `user_va`, read and
    // write, and they stay mapped until the claim is released; nothing
    // reaches the window after that, teardown being the last step.
    Ok(Regs::mmio(unsafe { Mmio::new(base as *mut u8, len) }))
}
