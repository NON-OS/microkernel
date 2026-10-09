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

//! The verdict's line on the serial log. A self-reported pass says so in
//! words, so it is never read as a measured one.

use nonos_boot_measure::gate::Admitted;

use super::verdict::Verdict;
use crate::sys::serial::{print, print_dec, println};

pub(super) fn say(v: Verdict) {
    match v {
        Verdict::Measured(a) => {
            admitted(b"[BOOT-ATTEST] bootloader measured and enrolled, epoch ", a)
        }
        Verdict::SelfReported(a) => {
            admitted(b"[BOOT-ATTEST] bootloader self-reported, not measured: enrolled, epoch ", a)
        }
        Verdict::Refused(code) => {
            print(b"[BOOT-ATTEST] bootloader refused, code ");
            print_dec(u64::from(code));
            println(b"");
        }
        Verdict::NoEvidence => {
            println(b"[BOOT-ATTEST] bootloader not checked: no boot-root record or trailer")
        }
    }
}

fn admitted(line: &[u8], a: Admitted) {
    print(line);
    print_dec(a.epoch);
    println(b"");
}
