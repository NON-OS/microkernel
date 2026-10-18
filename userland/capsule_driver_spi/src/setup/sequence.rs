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
//! Bring-up: each candidate claimed and its BAR mapped in turn, the first
//! proved an SPI host kept; every other claim is given back as it drops.

use alloc::vec::Vec;

use nonos_libc::{mk_idle_ms, mk_mmio_map, MmioMapOut};

use super::claim::{Claim, CLAIM_REFUSED};
use super::prove::{prove, ready};
use crate::discover::Found;
use crate::regs::Mmio;

pub struct Host {
    /// Held for as long as the host is served; dropping it gives it back.
    pub _claim: Claim,
    pub regs: Mmio,
    pub device: u16,
    /// The claim's device id with bit 63 set: the key the firmware's record
    /// and driver.gpio0 name this host by.
    pub key: u64,
}

/// Tries over which a candidate another driver holds is looked at again:
/// i2c_pci0 claims an unlisted LPSS function a moment to read its type.
const TRIES: u32 = 3;
const KEY_DEVICE_ID: u64 = 1 << 63;
const RETRY_MS: u64 = 1_000;

pub fn run(found: &Vec<Found>) -> Result<Host, &'static str> {
    let mut why = "spi: no candidate";
    for attempt in 0..TRIES {
        if attempt > 0 {
            let _ = mk_idle_ms(RETRY_MS);
        }
        for f in found {
            match up(f) {
                Ok(h) => return Ok(h),
                Err(e) => why = e,
            }
        }
        if why != CLAIM_REFUSED {
            break;
        }
    }
    Err(why)
}

fn up(f: &Found) -> Result<Host, &'static str> {
    let claim = Claim::take(f.device_id)?;
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let len = f.bar_size.min(0x1000);
    if mk_mmio_map(f.device_id, claim.epoch, 0, 0, 0, len, &mut out) < 0 {
        return Err("spi: BAR 0 would not map");
    }
    let regs = Mmio::new(out.user_va, out.length);
    prove(&regs)?;
    ready(&regs)?;
    let key = KEY_DEVICE_ID | f.device_id;
    Ok(Host { _claim: claim, regs, device: f.device, key })
}
