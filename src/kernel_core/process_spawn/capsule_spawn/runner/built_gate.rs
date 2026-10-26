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

//! Whether an embedded capsule is in this kernel at all, asked before its
//! cert is decoded. A capsule whose feature is off embeds empty consts, and
//! one built in with an empty file no longer compiles (each embed's
//! `const _: () = assert!`), so an empty ELF here means "not built". It was
//! asked only at install, after preflight, and a core kernel's every spawn
//! read as "NØNOS ID cert decode failed (UnexpectedEof)" (B-02). Generic and
//! free of the kernel's types, so kernel_proofs runs this very order.

/// `not_built` when `elf` is empty, else on.
pub(crate) fn gate<E>(elf: &[u8], not_built: E) -> Result<(), E> {
    if elf.is_empty() {
        return Err(not_built);
    }
    Ok(())
}

/// A verified spawn's checks in their order: the boot profile, then the
/// gate, and only then the preflight that decodes the cert and manifest.
pub(crate) fn checked<P, E>(
    elf: &[u8],
    not_built: E,
    profile: impl FnOnce() -> Result<(), E>,
    preflight: impl FnOnce() -> Result<P, E>,
) -> Result<P, E> {
    profile()?;
    gate(elf, not_built)?;
    preflight()
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
