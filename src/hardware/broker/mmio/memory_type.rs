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

//! The memory type a capsule's MMIO mapping gets; pure. Uncached by default,
//! since registers need every access in order and unmerged. Write-combining
//! only when asked (`MK_MMIO_MAP_WC`) and only on a prefetchable BAR, one the
//! function declares free of read side effects (PCI Local Bus 3.0, 6.2.5.1).
//! Where the kernel maps the range itself (the boot frame buffer), the capsule
//! gets the kernel's type whatever it asked: a page mapped with two memory
//! types is undefined (Intel SDM Vol. 3A, 12.12.4).

pub(crate) const MMIO_MAP_WC: u32 = 1 << 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MemType {
    Uncached,
    WriteCombining,
}

/// What the kernel's own mappings of the range say.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum KernelView {
    Unmapped,
    Mapped(MemType),
    Mixed, // two kernel mappings disagree: no capsule type is safe
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TypeRefusal {
    UnknownFlags,
    NotPrefetchable,
    Mixed,
}

pub(crate) const fn choose(
    flags: u32,
    prefetchable: bool,
    kernel: KernelView,
) -> Result<MemType, TypeRefusal> {
    if flags & !MMIO_MAP_WC != 0 {
        return Err(TypeRefusal::UnknownFlags);
    }
    let wc = flags & MMIO_MAP_WC != 0;
    if wc && !prefetchable {
        return Err(TypeRefusal::NotPrefetchable);
    }
    match kernel {
        KernelView::Mixed => Err(TypeRefusal::Mixed),
        KernelView::Mapped(t) => Ok(t),
        KernelView::Unmapped if wc => Ok(MemType::WriteCombining),
        KernelView::Unmapped => Ok(MemType::Uncached),
    }
}

/// Folds the types of every kernel mapping that touches the range.
pub(crate) fn fold(types: impl IntoIterator<Item = MemType>) -> KernelView {
    types.into_iter().fold(KernelView::Unmapped, |seen, t| match seen {
        KernelView::Unmapped => KernelView::Mapped(t),
        KernelView::Mapped(s) if s == t => seen,
        _ => KernelView::Mixed,
    })
}
