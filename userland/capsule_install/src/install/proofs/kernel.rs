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

//! The kernel's rows: its STARK proof and path under the enrolled kernel
//! root, and its release signature, as the bootloader recorded them before
//! the jump. The root and epoch come from the kernel's policy record.

use alloc::format;
use alloc::string::String;

use super::mark::Mark;
use super::row::Row;
use crate::install::format::hex_prefix;
use crate::install::source::Boot;

const KERNEL: &str = "KERNEL";
const SIGNATURE: &str = "KERNEL SIGNATURE";
const UNREPORTED: &str = "the kernel did not report the boot record";

/*
 * Both boot flags and the policy record come from the one gate, so on a real
 * boot they agree. A pass needs all three and the proof to name the kernel
 * that is running; any disagreement is a failure. The record cannot tell an
 * absent trailer from a failed one, since only a development boot runs past
 * either, so a boot with no pass on record claims neither.
 */
pub(super) fn kernel(b: &Boot) -> Row {
    if !b.available {
        return Row::new(KERNEL, Mark::Unknown, UNREPORTED, String::new());
    }
    let ran = format!("kernel {}", hex_prefix(&b.kernel_blake3));
    let claims = b.attested || b.proof;
    let pass = b.attested && b.proof && b.program_hash == b.kernel_blake3;
    match (b.policy.map(|p| p.kernel), claims) {
        (Some(Some(t)), true) if pass => {
            let says = if b.path_only {
                format!("path under the enrolled root, epoch {}; development, no STARK", t.epoch)
            } else {
                format!("STARK + path under the enrolled root, epoch {}", t.epoch)
            };
            Row::new(KERNEL, Mark::Verified, &says, format!("root {}  {ran}", hex_prefix(&t.root)))
        }
        (Some(None) | None, false) => {
            Row::new(KERNEL, Mark::NotVerified, "no STARK pass recorded: a development boot", ran)
        }
        (None, true) if pass => {
            Row::new(KERNEL, Mark::Unknown, "the kernel did not report its root", ran)
        }
        _ => Row::new(KERNEL, Mark::Failed, "the boot records disagree on what was proven", ran),
    }
}

pub(super) fn signature(b: &Boot) -> Row {
    let ran = format!("kernel {}", hex_prefix(&b.kernel_blake3));
    match (b.available, b.signature_ok) {
        (false, _) => Row::new(SIGNATURE, Mark::Unknown, UNREPORTED, String::new()),
        (true, true) if b.path_only => {
            Row::new(SIGNATURE, Mark::Verified, "signed by the keys this development loader holds", ran)
        }
        (true, true) => Row::new(SIGNATURE, Mark::Verified, "the release keys signed it", ran),
        (true, false) => {
            Row::new(SIGNATURE, Mark::NotVerified, "no valid release signature recorded", ran)
        }
    }
}
