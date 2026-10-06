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

//! The kernel's own check of the bootloader that started it, read back.
//!
//! The boot record is the bootloader's word about the kernel; this is the
//! kernel's word about the bootloader, from the firmware's log and the
//! loader's trailer. It was settled once, at boot, and reading it re-checks
//! nothing.

use nonos_libc::boot_attest::{boot_attest, AdmittedLoader, BootAttest};

use super::types::Verdict;

pub struct Loader {
    pub verdict: Verdict,
    pub claim: &'static [u8],
    /// What was admitted, when anything was.
    pub admitted: Option<AdmittedLoader>,
    /// The kernel's refusal code, when it refused.
    pub code: Option<u32>,
}

/*
 * Always a value: a kernel that does not answer is a row that says so. A
 * self-reported pass gets a dash, not a tick: the loader's own file is
 * enrolled, but nothing measured that it is what ran.
 */
pub fn loader() -> Loader {
    let (verdict, claim, admitted, code): (Verdict, &'static [u8], _, _) = match boot_attest() {
        Some(BootAttest::Measured(a)) => {
            (Verdict::Holds, b"bootloader measured and enrolled", Some(a), None)
        }
        Some(BootAttest::SelfReported(a)) => {
            (Verdict::Unknown, b"bootloader self-reported, not measured", Some(a), None)
        }
        Some(BootAttest::Refused(c)) => (Verdict::Broken, b"bootloader refused", None, Some(c)),
        Some(BootAttest::NoEvidence) => {
            (Verdict::Unknown, b"no boot evidence reached the kernel", None, None)
        }
        Some(BootAttest::NotYet) => (Verdict::Unknown, b"bootloader not yet checked", None, None),
        None => (Verdict::Unknown, b"the kernel did not answer", None, None),
    };
    Loader { verdict, claim, admitted, code }
}
