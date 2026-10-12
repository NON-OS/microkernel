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

//! The bring-up: the VMBus root claimed, one grant for the contact's
//! pages, the contact, and the offers. A failure gives back what was
//! taken, the claim last, and a host already contacted is told to forget
//! it first.

use nonos_libc::{
    mk_device_claim, mk_device_release, mk_dma_map, mk_dma_unmap, DmaMapOut, MK_DMA_MAP_COHERENT,
};

use super::driver::Driver;
use crate::connect::{self, Pages};
use crate::constants::{GRANT_BYTES, INTERRUPT_PAGE, MONITOR_PAGE_1, MONITOR_PAGE_2};
use crate::hv::Hv;
use crate::offers::Offers;

pub fn run(device_id: u64) -> Result<Driver, &'static str> {
    let epoch = mk_device_claim(device_id);
    if epoch < 0 {
        return Err("claim failed");
    }
    let epoch = epoch as u64;
    let release = |e: &'static str| {
        let _ = mk_device_release(device_id);
        e
    };
    let mut dma = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(device_id, epoch, GRANT_BYTES, MK_DMA_MAP_COHERENT, &mut dma) < 0 {
        return Err(release("dma map failed"));
    }
    let give_back = |e: &'static str| {
        let _ = mk_dma_unmap(dma.grant_id);
        release(e)
    };
    if dma.length < GRANT_BYTES {
        return Err(give_back("dma grant short"));
    }
    let hv = Hv { device_id };
    let pages = Pages {
        interrupt: dma.device_addr + INTERRUPT_PAGE,
        monitor1: dma.device_addr + MONITOR_PAGE_1,
        monitor2: dma.device_addr + MONITOR_PAGE_2,
    };
    let bus = connect::connect(&hv, pages).map_err(give_back)?;
    let mut offers = Offers::new();
    if let Err(e) = connect::offers(&hv, bus, &mut offers) {
        connect::disconnect(&hv, bus);
        return Err(give_back(e));
    }
    Ok(Driver { hv, bus, offers })
}
