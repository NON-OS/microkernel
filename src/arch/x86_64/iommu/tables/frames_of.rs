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

//! The frames a domain's page table is made of, pure: its root and
//! every table below it, never a leaf's frame, which is the DMA buffer a
//! grant owns. A destroyed domain gives these back once no unit can walk
//! them. kernel_proofs::iommu_attach.

extern crate alloc;

use alloc::vec::Vec;

/// Every table under `root` (a `levels`-deep table), children first.
/// `read` gives a table's entries; `child` the table an entry at a level
/// points to, None for a leaf or an empty entry. None when a table cannot
/// be read: then nothing is given back, and nothing is reused.
pub(crate) fn table_frames<R: AsRef<[u64]>>(
    root: u64,
    levels: u8,
    read: &impl Fn(u64) -> Option<R>,
    child: &impl Fn(u64, u8) -> Option<u64>,
) -> Option<Vec<u64>> {
    let mut out = Vec::new();
    if levels > 1 {
        let entries = read(root)?;
        for &entry in entries.as_ref() {
            if let Some(next) = child(entry, levels) {
                out.extend(table_frames(next, levels - 1, read, child)?);
            }
        }
    }
    out.push(root);
    Some(out)
}
