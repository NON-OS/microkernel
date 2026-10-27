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

//! What the screen says for each answer the kernel gives to a request to
//! prepare for an update. Paragraphs, wrapped where they are drawn; the
//! installer's layout proofs hold each to the first screen's body.

use alloc::string::String;

/// The words, and whether they say it is done.
pub fn answer(rc: i64) -> (String, bool) {
    let said = match rc {
        0 => "Ready. Restart from the new NØNOS stick and choose Update NØNOS (keep my data). Until you do, your data opens without the bootloader check for one start.",
        -95 => "This start keeps nothing, so there is nothing to keep.",
        -11 => "Your data volume is not open yet. Try again in a moment.",
        -2 => "This disk's data was made before 0.9.3 and cannot be kept.",
        -114 => "Your data is kept by a passphrase: nothing to prepare.",
        -13 => "This start is not one the release approved, so it cannot prepare.",
        -5 => "The disk did not take the change.",
        -1 => "The installer may not do this.",
        -38 => "This system cannot prepare for an update yet.",
        rc => return (alloc::format!("The kernel did not prepare it (errno {}).", -rc), false),
    };
    (String::from(said), rc == 0)
}
