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

//! `Firmware` over the broker (lead L31): the service registered on G's
//! bootfb port, the claim, `mk_boot_framebuffer`, bar 0 mapped
//! write-combined (the kernel keeps its own mapping's type), the release.

use nonos_libc::{
    mk_boot_framebuffer, mk_debug, mk_device_claim, mk_device_release, mk_mmio_map,
    mk_service_register, BootFramebufferInfo, MmioMapOut,
};

use crate::firmware::{Firmware, Why};
use crate::geometry::INFO_LEN;

/// driver.bootfb0's port, from G's 4570-4609.
pub const PORT: u32 = 4572;
const SERVICE: &[u8] = b"driver.bootfb0";
const MK_MMIO_MAP_WC: u32 = 1 << 0;
/// Registered already, by this process's spawn.
const EEXIST: i64 = -17;
/// The claim refused, final for the boot (L79).
const EPERM: i64 = -1;

pub struct Libc;

impl Firmware for Libc {
    fn announce(&mut self) -> Result<(), Why> {
        let rc = mk_service_register(SERVICE.as_ptr(), SERVICE.len(), PORT);
        (rc == 0 || rc == EEXIST).then_some(()).ok_or(Why::Announce)
    }
    fn claim(&mut self, device_id: u64) -> Result<u64, Why> {
        let rc = mk_device_claim(device_id);
        u64::try_from(rc).map_err(|_| if rc == EPERM { Why::Refused } else { Why::Claim })
    }
    fn geometry(&mut self, device_id: u64) -> Result<[u8; INFO_LEN], Why> {
        let mut info = BootFramebufferInfo::default();
        if mk_boot_framebuffer(device_id, &mut info) != INFO_LEN as i64 {
            return Err(Why::Geometry);
        }
        let f = [info.width, info.height, info.stride, info.format, info.offset, info.phys_mm];
        let mut b = [0u8; INFO_LEN];
        for (i, v) in f.iter().enumerate() {
            b[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        Ok(b)
    }
    fn map(&mut self, device_id: u64, epoch: u64, size: u64) -> Result<*mut u8, Why> {
        let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
        let rc = mk_mmio_map(device_id, epoch, 0, MK_MMIO_MAP_WC, 0, size, &mut out);
        let whole = rc >= 0 && out.user_va != 0 && out.length >= size;
        whole.then_some(out.user_va as *mut u8).ok_or(Why::Map)
    }
    fn release(&mut self, device_id: u64) {
        let _ = mk_device_release(device_id);
    }
    fn say(&mut self, line: &str) {
        let _ = mk_debug(line.as_ptr(), line.len());
    }
}
