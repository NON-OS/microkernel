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

//! The steps of an attach, pure: write the context entry, then drop
//! the caches. When the drop fails after the write, the entry comes out
//! again and the caches are dropped once more, so no entry the kernel does
//! not record stays live; the caller forgets the binding on any error.
//! kernel_proofs::iommu_attach.

/// The first error, after the entry was taken back out if it was written.
pub(crate) fn attach<E>(
    set: impl FnOnce() -> Result<(), E>,
    mut invalidate: impl FnMut() -> Result<(), E>,
    clear: impl FnOnce() -> Result<(), E>,
) -> Result<(), E> {
    set()?;
    if let Err(e) = invalidate() {
        if clear().is_ok() {
            let _ = invalidate();
        }
        return Err(e);
    }
    Ok(())
}
