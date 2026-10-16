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

//! The firmware entry the kernel files under pid 0: About lists it, counts
//! it, and lets it admit nothing, as firmware and never as a running program.

use crate::proof_board::census::{census, Admitted, Proc, CAP_NETWORK};
use crate::proof_board::firmware::{
    counted, firmware, is_firmware, programs, FIRMWARE_LABEL, FIRMWARE_PID, FIRMWARE_WHEN,
};

const RUNNING: u8 = 2;

fn a(pid: u32, caps: u64) -> Admitted {
    Admitted { pid, caps, authority: 0 }
}

#[test]
fn only_pid_zero_is_the_firmware() {
    assert_eq!((FIRMWARE_PID, FIRMWARE_LABEL), (0, &b"firmware tables"[..]));
    assert_eq!(FIRMWARE_WHEN, b"measured before any _INI");
    assert!(is_firmware(0));
    assert!([1, 2, 255, u32::MAX].iter().all(|&p| !is_firmware(p)));
}

#[test]
fn a_pid_zero_entry_is_never_among_the_programs() {
    let reg = [a(0, 0), a(2, 0x19), a(0, 0x1D), a(7, 0)];
    let pids: Vec<u32> = programs(&reg, |e| e.pid).map(|e| e.pid).collect();
    assert_eq!(pids, [2, 7]);
    assert_eq!(firmware(&reg, |e| e.pid).map(|e| e.caps), Some(0));
    assert!(firmware(&reg[1..2], |e| e.pid).is_none());
    assert_eq!(programs(&[] as &[Admitted], |e| e.pid).count(), 0);
}

#[test]
fn the_firmware_entry_admits_no_process() {
    // Should the table ever show a pid 0 row, the measurement of the tables
    // must not vouch for it.
    let procs = [
        Proc { pid: 0, state: RUNNING, uptime_ms: 60_000, caps: 0x19, name: b"idle" },
        Proc { pid: 2, state: RUNNING, uptime_ms: 60_000, caps: 0x19, name: b"vfs" },
    ];
    let c = census(&procs, &[a(0, CAP_NETWORK), a(2, 0x19)]);
    assert_eq!((c.running, c.admitted, c.network), (2, 1, 0));
    assert_eq!((c.unadmitted, c.first_unadmitted), (1, Some(0)));
}

#[test]
fn the_firmware_entry_alone_counts_as_no_running_capsule() {
    let procs = [Proc { pid: 2, state: RUNNING, uptime_ms: 60_000, caps: 0x19, name: b"vfs" }];
    let c = census(&procs, &[a(0, 0), a(2, 0x19)]);
    assert_eq!((c.running, c.admitted, c.unadmitted), (1, 1, 0));
}

#[test]
fn the_documents_count_names_capsules_and_firmware_apart() {
    assert_eq!(counted(45, Some(true)), (44, &b" capsules and the firmware tables over "[..]));
    assert_eq!(counted(44, Some(false)), (44, &b" capsules over "[..]));
    assert_eq!(counted(45, None), (45, &b" registry entries over "[..]));
    assert_eq!(counted(0, Some(true)), (0, &b" registry entries over "[..]));
}
