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

use spin::Once;

use crate::arch::fdt::find::mailbox::{find, MailboxInfo};
use crate::arch::fdt::{Fdt, FdtError};
use crate::memory::addr::PhysAddr;

static INFO: Once<MailboxInfo> = Once::new();
static REGS: Once<Option<u64>> = Once::new();

/// Keep the tree's mailbox, if it has one; boot calls this once.
pub fn describe(fdt: &Fdt) -> Result<(), FdtError> {
    if let Some(info) = find(fdt)? {
        INFO.call_once(|| info);
    }
    Ok(())
}

/// The mailbox and its registers' kernel address, mapped on first use: the
/// tree is read before the kernel can map anything.
pub(super) fn mailbox() -> Option<(&'static MailboxInfo, u64)> {
    let info = INFO.get()?;
    let regs = REGS.call_once(|| {
        let size = info.size.max(0x40) as usize;
        let va = crate::memory::mmio::map_device_memory(PhysAddr::new(info.base), size).ok();
        va.map(|va| va.as_u64())
    });
    Some((info, (*regs)?))
}
