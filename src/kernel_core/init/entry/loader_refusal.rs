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

use crate::security::boot::loader_check::Verdict;
use crate::sys::boot_log;

/*
 * The kernel's check of the bootloader is enforced: a loader it refused, or a
 * boot that brought no boot-root record or loader trailer to check, starts no
 * userspace. A loader admitted without a TPM to measure it was still held to
 * its STARK under the signed root, and its verdict says it was not measured.
 * Runs before init, so nothing has started that the refusal would undo.
 */
pub(super) fn refuse_unchecked_loader(v: Verdict) {
    let (title, lines): (&[u8], [&[u8]; 2]) = match v {
        Verdict::Measured(_) | Verdict::SelfReported(_) => return,
        Verdict::Refused(_) => (
            b"The bootloader failed the kernel's check",
            [
                b"Its measurement, its boot-root record or its STARK proof did not verify.",
                b"No program was started. Boot an image whose loader is enrolled.",
            ],
        ),
        Verdict::NoEvidence => (
            b"The bootloader could not be checked",
            [
                b"This boot carried no boot-root record or no bootloader trailer to check.",
                b"No program was started. Boot an image built with its boot evidence.",
            ],
        ),
    };
    boot_log::show_notice(title, &lines);
    boot_log::error("bootloader not admitted by the kernel's check; stopping");
    crate::arch::halt_loop()
}
