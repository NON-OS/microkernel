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

//! What the guest's first listen is answered, and what the person is told,
//! when the kernel will not serve its listener (MkServeOne). EPERM is the
//! kernel's word for a caller that is not a run role or a listing SERVES
//! does not pair with the capsule: the policy did grant SERVE_ONE, so the
//! line says which was refused rather than that the policy lacks it.

use alloc::format;
use alloc::string::String;

/// The kernel's answers this file reads, as negative errno values.
const EPERM: i64 = 1;
const ENOMEM: i64 = 12;
const EBUSY: i64 = 16;

/// Why MkServeOne gave no port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The capsule is served already, or this process serves one: EADDRINUSE.
    InUse,
    /// The registry is full: ENOBUFS.
    NoRoom,
    /// Not a run role, or a listing the kernel does not pair with the capsule.
    NotPaired,
    /// Any other answer: a name or key the kernel would not read.
    Refused,
}

/// The refusal for MkServeOne's answer `rc`, which is no port (`rc` <= 0).
pub fn of(rc: i64) -> Refusal {
    match rc.checked_neg() {
        Some(EBUSY) => Refusal::InUse,
        Some(ENOMEM) => Refusal::NoRoom,
        Some(EPERM) => Refusal::NotPaired,
        _ => Refusal::Refused,
    }
}

/// The line for a NotPaired listen of the listener served to `capsule`.
pub fn not_paired(capsule: &str) -> String {
    let shown: String =
        capsule.chars().map(|c| if c.is_ascii_graphic() { c } else { '?' }).collect();
    format!(
        "[LINUX] refused listen: the kernel serves {shown} only to a store run of the listing \
         it pairs with it, and this run is not one\n"
    )
}
