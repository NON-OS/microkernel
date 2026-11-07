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

//! The serial line nsh.front writes for each refusal or end the server
//! reports (`Event::Refused`). nsh/1 tells the client one reason code at
//! most, and nothing at all for some refusals; this line is where an
//! operator reads the server's whole `Why`. A `Why` names kinds of fault
//! only, never bytes the peer sent, so the line carries nothing from the
//! peer. Pure, so nsh_front_proofs holds it.

use alloc::format;
use alloc::vec::Vec;
use core::fmt::Debug;

/// The line for one refusal, newline ended.
pub fn line(why: &impl Debug) -> Vec<u8> {
    format!("[NSH-FRONT] refused or ended: {why:?}\n").into_bytes()
}
