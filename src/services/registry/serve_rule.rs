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

//! What MkServeOne admits, pure: the run role, for a pair SERVES lists, one
//! served endpoint a process. The endpoint is `linux.serve.<capsule>` on its
//! row's port, held to the capsule (held_gate.rs). Held in kernel_proofs.

use alloc::{format, string::String};

// Mounted by path, so kernel_proofs, which mounts this file alone, reads it.
#[path = "serves.rs"]
pub(crate) mod serves;
use serves::SERVES;

/// The served endpoints' root, which no loaded capsule declares under
/// (baked_names.rs CORE).
pub(crate) const SERVE_PREFIX: &str = "linux.serve.";
/// A capsule's name, as a policy's SERVE_ONE field holds it: 1 to 32 bytes.
pub(crate) const SERVE_NAME_MAX: usize = 32;
/// SERVES' first row is served on this port, each next row on the next.
const SERVE_PORT_BASE: u32 = 5300;

/// Why MkServeOne refuses: a name not 1 to 32 bytes of `[a-z0-9._-]`, a letter
/// first (EINVAL); not the run role, or a pair SERVES does not list (EPERM).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ServeRefusal {
    BadName,
    NotTheRun,
    NotPaired,
}

/// The SERVES row a caller may serve, given the capsule `name` and listing
/// `key` it named, and whether it is the run `role`.
pub(crate) fn admit_serve(name: &[u8], key: &[u8; 32], role: bool) -> Result<usize, ServeRefusal> {
    let form = |(i, b): (usize, &u8)| {
        b.is_ascii_lowercase() || i > 0 && (b.is_ascii_digit() || b"._-".contains(b))
    };
    if name.is_empty() || name.len() > SERVE_NAME_MAX || !name.iter().enumerate().all(form) {
        return Err(ServeRefusal::BadName);
    }
    if !role {
        return Err(ServeRefusal::NotTheRun);
    }
    SERVES.iter().position(|(n, k)| n.as_bytes() == name && k == key).ok_or(ServeRefusal::NotPaired)
}

/// Whether a process owning the endpoints `owned` may be given a served
/// one: only while it serves none, so a run serves one listener.
pub(crate) fn serves_none<'a>(mut owned: impl Iterator<Item = &'a str>) -> bool {
    !owned.any(|name| name.starts_with(SERVE_PREFIX))
}

/// Row `i`'s endpoint and port.
pub(crate) fn served_endpoint(i: usize) -> (String, u32) {
    (format!("{SERVE_PREFIX}{}", SERVES[i].0), SERVE_PORT_BASE + i as u32)
}

/// Whether `port` is a served endpoint's, which only MkServeOne registers.
pub(crate) fn is_serve_port(port: u32) -> bool {
    port.checked_sub(SERVE_PORT_BASE).is_some_and(|i| (i as usize) < SERVES.len())
}
