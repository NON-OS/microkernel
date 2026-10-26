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

//! `mk_hv_backdoor` as the kernel serves it: -19 on a host without a
//! backdoor, -22 for a call outside the kernel's own allowlist (included
//! here by path); otherwise the host the test installs answers, handed the
//! registers as the kernel loads them.

use std::cell::RefCell;

use crate::backdoor_rule::{answered, call_allowed, host_answers};
use crate::backdoor_rule::{BACKDOOR_MAGIC, BACKDOOR_PORT, CMD_GETVERSION};
use crate::vendor::Hypervisor;

/// The backdoor's other side: EAX, EBX, ECX and EDX in, as the kernel loads
/// them for one allowed call, and as the host leaves them out.
pub type Host = Box<dyn FnMut([u32; 4]) -> [u32; 4]>;

thread_local! {
    static HOST: RefCell<Option<(Hypervisor, Host)>> = const { RefCell::new(None) };
    static CALLS: RefCell<Vec<(u32, u32)>> = const { RefCell::new(Vec::new()) };
}

/// The hypervisor CPUID named, by its code, and what answers its backdoor.
pub fn set_host(vendor_code: u8, host: Host) {
    HOST.with(|h| *h.borrow_mut() = Some((Hypervisor::from_code(vendor_code), host)));
}

/// Every call that reached the host on this thread, oldest first.
pub fn backdoor_calls() -> Vec<(u32, u32)> {
    CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()))
}

/// The kernel's own look, before it starts a driver here: GETVERSION answered.
pub fn backdoor_present() -> bool {
    matches!(backdoor(CMD_GETVERSION, 0), Ok(regs) if answered(regs))
}

fn backdoor(cmd: u32, arg: u32) -> Result<[u32; 4], i64> {
    HOST.with(|h| {
        let mut h = h.borrow_mut();
        let Some((vendor, host)) = h.as_mut() else { return Err(-19) };
        if !host_answers(*vendor) {
            return Err(-19);
        }
        if !call_allowed(cmd, arg) {
            return Err(-22);
        }
        Ok(host([BACKDOOR_MAGIC, arg, cmd, BACKDOOR_PORT as u32]))
    })
}

pub fn mk_hv_backdoor(cmd: u32, arg: u32, out: *mut [u32; 4]) -> i64 {
    let regs = match backdoor(cmd, arg) {
        Ok(regs) => regs,
        Err(e) => return e,
    };
    CALLS.with(|c| c.borrow_mut().push((cmd, arg)));
    // SAFETY: the caller hands a live [u32; 4], as the real call requires.
    unsafe { *out = regs };
    0
}
