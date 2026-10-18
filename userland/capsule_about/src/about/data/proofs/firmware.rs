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

//! The one registry entry that is not a program.
//!
//! Before any AML method runs, the kernel folds a BLAKE3 digest of every
//! firmware table into the attestation registry under pid 0, which no process
//! ever holds, so the registry root moves when the machine's firmware does.
//! Every screen that lists the registry asks here first, and the entry is
//! drawn as the firmware it measures, never as a capsule that is running.
//! Pure, so the rule is proven on the host (attest_doc_proofs).

/// The pid the kernel files the firmware measurement under.
pub const FIRMWARE_PID: u32 = 0;
/// What the entry is called wherever it is shown.
pub const FIRMWARE_LABEL: &[u8] = b"firmware tables";
/// When it was taken, which is what makes it worth trusting.
pub const FIRMWARE_WHEN: &[u8] = b"measured before any _INI";

pub fn is_firmware(pid: u32) -> bool {
    pid == FIRMWARE_PID
}

/// The entries that are running programs, in the order given.
pub fn programs<T>(list: &[T], pid: impl Fn(&T) -> u32) -> impl Iterator<Item = &T> {
    list.iter().filter(move |e| !is_firmware(pid(e)))
}

/// The firmware entry, if the registry lists one.
pub fn firmware<T>(list: &[T], pid: impl Fn(&T) -> u32) -> Option<&T> {
    list.iter().find(|e| is_firmware(pid(e)))
}

/// The number to print for a signed document's entry count and the words
/// after it. `listed` is whether the registry read now holds the firmware
/// entry, `None` when it could not be read; then the count is given as
/// entries, since calling all of them capsules might be false.
pub fn counted(count: u32, listed: Option<bool>) -> (u32, &'static [u8]) {
    match listed {
        Some(true) if count > 0 => (count - 1, b" capsules and the firmware tables over "),
        Some(false) => (count, b" capsules over "),
        _ => (count, b" registry entries over "),
    }
}
