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

//! Actions and masks never touch SIGKILL or SIGSTOP.

use super::super::fake_memory::Fake;
use crate::linux::abi::errno::{EFAULT, EINVAL};
use crate::linux::call::signal_act::{decode, encode, signal_of, wait_mask, wait_set};
use crate::linux::guest::sigstate::SigAction;

#[test]
fn rt_sigaction_names_an_int_signal_and_never_changes_kill_or_stop() {
    assert_eq!(signal_of(0, false), Err(EINVAL));
    assert_eq!(signal_of(65, false), Err(EINVAL));
    assert_eq!(signal_of(u64::MAX, false), Err(EINVAL));
    assert_eq!(signal_of(1, true), Ok(1));
    assert_eq!(signal_of(64, true), Ok(64));
    assert_eq!(signal_of(9, true), Err(EINVAL));
    assert_eq!(signal_of(19, true), Err(EINVAL));
    assert_eq!(signal_of(9, false), Ok(9), "SIGKILL's action may be read");
    assert_eq!(signal_of((1 << 32) | 2, true), Ok(2), "the signal is an int");
}

#[test]
fn a_stored_action_never_blocks_kill_or_stop() {
    let a = SigAction { handler: 0x40_1000, flags: 0x0400_0000, restorer: 0x40_2000, mask: !0 };
    let back = decode(&encode(a));
    let want = !((1u64 << 8) | (1 << 18));
    assert_eq!(
        back.map(|b| (b.handler, b.flags, b.restorer, b.mask)),
        Some((a.handler, a.flags, a.restorer, want))
    );
    assert!(decode(&[0u8; 31]).is_none());
}

#[test]
fn a_waiting_call_s_mask_is_read_as_set_user_sigmask_reads_it() {
    let mem = Fake::new(0x4000, 0x40);
    mem.put(0x4000, &u64::MAX.to_le_bytes());
    assert_eq!(wait_mask(&mem, 0, 99), Ok(None), "no mask: the size is not looked at");
    assert_eq!(wait_mask(&mem, 0x4000, 16), Err(EINVAL));
    assert_eq!(wait_mask(&mem, 0x1000, 8), Err(EFAULT));
    assert_eq!(wait_mask(&mem, 0x403c, 8), Err(EFAULT), "half of it outside");
    let all_but = !((1u64 << 8) | (1 << 18));
    assert_eq!(wait_mask(&mem, 0x4000, 8), Ok(Some(all_but)), "never SIGKILL or SIGSTOP");
}

#[test]
fn sigtimedwait_never_takes_sigkill_or_sigstop() {
    let mem = Fake::new(0x4000, 0x10);
    mem.put(0x4000, &u64::MAX.to_le_bytes());
    let set = wait_set(&mem, 0x4000);
    assert_eq!(set.map(|s| s & (1 << 8)), Some(0), "SIGKILL ends the process, never a wait");
    assert_eq!(set.map(|s| s & (1 << 18)), Some(0), "SIGSTOP likewise");
    assert_eq!(set.map(|s| s.count_ones()), Some(62));
    assert_eq!(wait_set(&mem, 0x400c), None);
}
