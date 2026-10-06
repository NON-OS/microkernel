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

//! Which namespace to serve. NSID 1 is only a convention: a drive that was
//! reformatted, or one carved into namespaces by a vendor tool, can have its
//! first active namespace anywhere up to NN, and an inactive NSID identifies
//! as all zeros, which reads as an empty disk. The Identify CNS 02h page
//! lists the active NSIDs in ascending order, zero padded. Pure, as the page
//! is the controller's to write.

/// Identifiers that never name one namespace: 0 ends the list, all ones is
/// the broadcast NSID.
const BROADCAST_NSID: u32 = 0xffff_ffff;

/// The first NSID on an active namespace list page that can be a namespace:
/// not broadcast and, when the controller reports NN, not above it. A zero
/// entry ends the list; an empty list is None, a controller with no active
/// namespace.
pub fn first_active_nsid(page: &[u8], namespace_count: u32) -> Option<u32> {
    for entry in page.chunks_exact(4) {
        let nsid = u32::from_le_bytes([entry[0], entry[1], entry[2], entry[3]]);
        if nsid == 0 {
            return None;
        }
        if nsid == BROADCAST_NSID || (namespace_count != 0 && nsid > namespace_count) {
            continue;
        }
        return Some(nsid);
    }
    None
}

/// The NSID tried when the controller cannot list its namespaces (NVMe 1.0
/// has no CNS 02h): the first one, which every such drive ships with.
pub const FALLBACK_NSID: u32 = 1;

/// Whether a controller reporting version `vs` (VS register layout: major
/// in bits 31:16, minor in 15:8) knows CNS 02h, added in NVMe 1.1. A
/// version of zero is a pre-1.2 controller that left the field clear and is
/// asked anyway, falling back when it refuses.
pub const fn lists_active_namespaces(vs: u32) -> bool {
    let major = vs >> 16;
    let minor = (vs >> 8) & 0xff;
    vs == 0 || major > 1 || (major == 1 && minor >= 1)
}
