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

//! What this capsule's own manifest asks for, apart from the table of every
//! bit, so the one mask the Verify screen tests is read in one place.

use super::caps::{ATTEST_READ, CORE_EXEC, GFX_DISPLAY_QUERY, GFX_SURFACE_CREATE, IPC, MEMORY};

/*
 * A declaration, not a measurement: the kernel is the only party that knows
 * what was actually granted, and the Verify screen holds this constant up
 * against the mask the kernel recorded for our pid. If the two ever part, the
 * check fails and says so, which is the only way a hardcoded mask earns its
 * place in a build. AttestRead is for the attestation document and the list
 * of capsules the kernel proved at spawn. Capsule.mk declares the same bits.
 */
pub const MASK: u64 =
    CORE_EXEC | IPC | MEMORY | GFX_DISPLAY_QUERY | GFX_SURFACE_CREATE | ATTEST_READ;
