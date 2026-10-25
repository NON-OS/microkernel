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

use nonos_policy_proto::Field;

use super::kernel_field::{KEEP_ACROSS_BOOTS, KERNEL_PREEMPT, KIND_BOOL};
use super::raw;

pub fn on_bool_set(field: Field, value: bool) {
    match field {
        Field::KernelPreempt => {
            let _ = raw::submit(KERNEL_PREEMPT, KIND_BOOL, &[value as u8]);
        }
        /* Only the turn on is told: the kernel never turns keeping off. */
        Field::Persistent if value => keep_on(),
        _ => {}
    }
}

/*
 * Said on serial when the kernel refuses, as the store then stays shut while
 * vfs believes it open, and every keep fails until the reason is read.
 */
fn keep_on() {
    if raw::submit(KEEP_ACROSS_BOOTS, KIND_BOOL, &[1]) != 0 {
        let line = b"[POLICY] the kernel refused to turn keeping on\n";
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    }
}
