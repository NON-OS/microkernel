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

//! Whether a capsule image is there at all. A capsule whose feature is off
//! embeds empty consts, and one built in with an empty file no longer
//! compiles (each embed's `const _: () = assert!`, B-02); a verified spawn
//! says FeatureDisabled for one before its cert is decoded (verified.rs).
//! Here, a store load refuses an empty image read from the store by its own
//! name. Generic and free of the kernel's types, so kernel_proofs runs it.

/// `not_built` when `elf` is empty, else on.
pub(crate) fn gate<E>(elf: &[u8], not_built: E) -> Result<(), E> {
    if elf.is_empty() {
        return Err(not_built);
    }
    Ok(())
}

/// A store load's order: an empty image read from the store is refused
/// with `empty` before anything is spawned, so "not built" is only ever
/// said of an embedded capsule.
pub(crate) fn stored<P, E>(
    elf: &[u8],
    empty: E,
    spawn: impl FnOnce() -> Result<P, E>,
) -> Result<P, E> {
    gate(elf, empty)?;
    spawn()
}
