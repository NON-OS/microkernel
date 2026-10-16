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

//! The two DMA grants: the receive buffer and the four transmit slots. The
//! part's RBSTART and TSAD registers are 32 bits wide, so both are asked for
//! below 4 GiB, and each is held to ending there too: a grant that started
//! below and ran past would have the part's addresses wrap into low memory
//! that is not this driver's. A failing step gives the device back through
//! `rollback`, whose release frees whatever was granted.

use nonos_libc::{mk_dma_map, DmaMapOut, PioGrantOut, MK_DMA_MAP_DMA32};

use crate::constants::dma::{RX_BUF_BYTES, TX_BUF_BYTES};

use super::rollback;

const PAGE_MASK: u64 = 0xFFF;
const FOUR_GIB: u64 = 1 << 32;

fn page_round(n: u64) -> u64 {
    (n + PAGE_MASK) & !PAGE_MASK
}

fn alloc(
    device_id: u64,
    epoch: u64,
    bytes: u64,
    refused: &'static str,
) -> Result<DmaMapOut, &'static str> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(device_id, epoch, page_round(bytes), MK_DMA_MAP_DMA32, &mut out) < 0 {
        return Err(refused);
    }
    if out.length < bytes {
        return Err("rtl8139 dma grant shorter than asked");
    }
    let end = out.device_addr.checked_add(out.length);
    if end.is_none_or(|end| end > FOUR_GIB) {
        return Err("rtl8139 requires 32-bit dma");
    }
    Ok(out)
}

pub fn map_all(
    device_id: u64,
    epoch: u64,
    pio: &PioGrantOut,
) -> Result<(DmaMapOut, DmaMapOut), &'static str> {
    let give_back = |e| {
        rollback::after_pio(device_id, epoch, pio);
        e
    };
    let rx = alloc(device_id, epoch, RX_BUF_BYTES as u64, "rx dma failed").map_err(give_back)?;
    let tx = alloc(device_id, epoch, TX_BUF_BYTES as u64, "tx dma failed").map_err(give_back)?;
    Ok((rx, tx))
}
