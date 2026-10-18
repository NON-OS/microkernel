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

//! Every op answers only the services named for it: each other service
//! the endpoint admits, a sender owning nothing, and pid 0 are refused,
//! and an op nobody is named for refuses everyone.

use crate::policy::{admits, allowed, Allowed};

/// Every service the kernel's HELD line admits to the endpoint, plus the
/// ones the per-op lists name, each running under its own pid.
const SERVICES: &[&[u8]] = &[
    b"power",
    b"desktop_shell",
    b"driver.i2c_pci0",
    b"driver.i2c_hid0",
    b"driver.ps2_kbd0",
    b"driver.gpio0",
];

/// Every pid its own process: no threads.
fn alone(_: u32) -> Option<u32> {
    None
}

fn pid_of(name: &[u8]) -> Option<u32> {
    SERVICES.iter().position(|s| *s == name).map(|i| 100 + i as u32)
}

#[test]
fn each_op_answers_its_services_and_no_other() {
    for op in 0..=255u8 {
        for (i, name) in SERVICES.iter().enumerate() {
            let sender = 100 + i as u32;
            let want = match allowed(op) {
                Allowed::Services(names) => names.contains(name),
                Allowed::AnyAdmitted => true,
                Allowed::Nobody => false,
            };
            assert_eq!(admits(op, sender, pid_of, alone), want, "op {op} from {:?}", name);
        }
        assert!(!admits(op, 999, pid_of, alone) || allowed(op) == Allowed::AnyAdmitted, "op {op}");
        assert!(!admits(op, 0, pid_of, alone), "pid 0 never, op {op}");
    }
}

#[test]
fn a_service_no_one_runs_admits_no_one() {
    let nobody_runs = |_: &[u8]| None;
    let pid_zero = |_: &[u8]| Some(0);
    for op in [3, 5, 8, 9, 11] {
        assert!(!admits(op, 100, nobody_runs, alone), "op {op}");
        assert!(!admits(op, 100, pid_zero, alone), "op {op}: an owner of pid 0 is no owner");
    }
}
