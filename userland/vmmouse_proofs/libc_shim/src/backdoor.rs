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

//! `mk_hv_backdoor` as the kernel serves it: refused with -19 on a host
//! without a backdoor and with -22 for a call outside the kernel's own
//! allowlist, which is included here by path; otherwise answered by the
//! host the test installs.

use std::cell::RefCell;

use crate::backdoor_rule::{call_allowed, host_answers};
use crate::vendor::Hypervisor;

/// The backdoor's other side: the four registers for one allowed call.
pub type Host = Box<dyn FnMut(u32, u32) -> [u32; 4]>;

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

pub fn mk_hv_backdoor(cmd: u32, arg: u32, out: *mut [u32; 4]) -> i64 {
    HOST.with(|h| {
        let mut h = h.borrow_mut();
        let Some((vendor, host)) = h.as_mut() else { return -19 };
        if !host_answers(*vendor) {
            return -19;
        }
        if !call_allowed(cmd, arg) {
            return -22;
        }
        CALLS.with(|c| c.borrow_mut().push((cmd, arg)));
        let regs = host(cmd, arg);
        // SAFETY: the caller hands a live [u32; 4], as the real call requires.
        unsafe { *out = regs };
        0
    })
}
