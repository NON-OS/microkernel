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

//! Claim the host, turn on memory decoding and bus mastering (and quiet its
//! INTx pin: completions are polled), map BAR0, and take the four DMA
//! regions. A refusal anywhere drops what was taken, which gives it back.

use nonos_libc::{
    mk_device_claim, mk_mmio_map, mk_pci_config_write, MmioMapOut, MK_PCI_CFG_COMMAND,
    MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE,
};

use super::discover::Found;
use super::env::HostEnv;
use super::grants::{Claim, DmaRegion};
use super::held::Held;
use super::say::{host, refused};
use crate::ufs::utp::layout::UCD_LEN;

const PAGE: u64 = 4096;
// The command descriptor, both UPIUs and the PRDT entry, fits its page.
const _: () = assert!(UCD_LEN as u64 <= PAGE);
/// The data region: what one wire request moves at most. 64 KiB is 16
/// blocks of a 4 KiB unit and fits one PRDT entry (up to 256 KiB).
pub const DATA_BYTES: u64 = 64 * 1024;

pub fn take(dev: &Found) -> Option<(HostEnv, Held)> {
    host(dev.vendor, dev.device);
    let epoch = mk_device_claim(dev.device_id);
    if epoch < 0 {
        refused(b"claim", epoch);
        return None;
    }
    let mut claim = Claim { device_id: dev.device_id, epoch: epoch as u64, mmio_grant: None };
    let cmd = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
    let r = mk_pci_config_write(dev.device_id, claim.epoch, MK_PCI_CFG_COMMAND, cmd);
    if r < 0 {
        refused(b"PCI command write", r);
        return None;
    }
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let r = mk_mmio_map(dev.device_id, claim.epoch, 0, 0, 0, dev.bar0_len, &mut out);
    if r < 0 {
        refused(b"BAR0 map", r);
        return None;
    }
    claim.mmio_grant = Some(out.grant_id);
    // SAFETY: the grant just made covers BAR0, a page or more, and lives in
    // the claim, which the caller holds past the last register access.
    let env = unsafe { HostEnv::new(out.user_va) };
    let utrl = DmaRegion::map(&claim, PAGE, b"DMA map (UTRL)")?;
    let utmrl = DmaRegion::map(&claim, PAGE, b"DMA map (UTMRL)")?;
    let ucd = DmaRegion::map(&claim, PAGE, b"DMA map (UCD)")?;
    let data = DmaRegion::map(&claim, DATA_BYTES, b"DMA map (data)")?;
    Some((env, Held { regions: [utrl, utmrl, ucd, data], _claim: claim }))
}
