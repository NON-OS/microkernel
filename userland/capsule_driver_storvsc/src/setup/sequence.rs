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

//! The bring-up: the channel's record claimed, one DMA grant for the rings
//! and the data buffer, the channel opened on the rings, the protocol
//! started; then the targets scanned. Nothing is bound: the driver polls
//! the host's ring.

use nonos_libc::{mk_device_claim, mk_device_release, mk_dma_map, DmaMapOut, MK_DMA_MAP_COHERENT};
use nonos_scsi_serve::Units;
use nonos_vmbus::Pages;

use super::driver::{abandon, Driver};
use crate::constants::channel::{gpadl_id, MAX_LUN, MAX_TARGET, OPEN_USER};
use crate::constants::layout::{DATA_OFF, GRANT_BYTES, PAGE, RINGS, RING_BYTES};
use crate::discover::Found;
use crate::hba::VmHba;
use crate::host::Kernel;
use crate::start;
use crate::{NAME, REPLY_ENDPOINT};

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let epoch = mk_device_claim(dev.device_id);
    if epoch < 0 {
        return Err("claim failed");
    }
    let mut dma = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(dev.device_id, epoch as u64, GRANT_BYTES, MK_DMA_MAP_COHERENT, &mut dma) < 0 {
        let _ = mk_device_release(dev.device_id);
        return Err("dma map failed");
    }
    if dma.length < GRANT_BYTES || dma.device_addr % PAGE as u64 != 0 {
        abandon(dev.device_id, dma.grant_id);
        return Err("dma grant short");
    }
    let first_pfn = dma.device_addr / PAGE as u64;
    // SAFETY: the grant's first RING_BYTES, mapped for the driver's life
    // and reached by no one else but the host.
    let mut rings = unsafe { Pages::new(dma.user_va as *mut u8, RING_BYTES) };
    let mut host = Kernel::new(dev.device_id);
    let gpadl = gpadl_id(dev.relid);
    let chan = match nonos_vmbus::open(&mut host, &mut rings, dev.relid, gpadl, first_pfn, RINGS, &OPEN_USER) {
        Ok(c) => c,
        Err(_) => {
            abandon(dev.device_id, dma.grant_id);
            return Err("channel did not open");
        }
    };
    let data = (dma.user_va as usize + DATA_OFF) as *mut u8;
    let data_pfn = first_pfn + (DATA_OFF / PAGE) as u64;
    // SAFETY: the grant's data buffer, after the rings, as above.
    let mut hba = unsafe { VmHba::new(host, rings, chan, data, data_pfn) };
    if let Err(e) = start::run(&mut hba) {
        let _ = hba.chan.close(&mut hba.host);
        abandon(dev.device_id, dma.grant_id);
        return Err(e);
    }
    let mut units = Units::new(hba, REPLY_ENDPOINT, NAME);
    units.scan(MAX_TARGET, MAX_LUN);
    Ok(Driver { units, device_id: dev.device_id, dma_grant: dma.grant_id })
}
