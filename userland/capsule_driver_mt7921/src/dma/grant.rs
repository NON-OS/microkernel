/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Coherent memory the chip reaches, as the broker maps it: below 4 GiB,
//! since the WFDMA takes 32-bit addresses (`dma_set_mask(32)` in
//! `mt7921_pci_probe`). The accesses are in `access`.

use nonos_libc::{mk_dma_map, DmaMapOut, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

#[derive(Clone, Copy)]
pub struct Grant {
    pub(super) va: usize,
    pub(super) dev: u64,
    pub(super) len: usize,
}

impl Grant {
    /// `bytes` rounded up to pages, mapped for the claim, or `None` when the
    /// broker refused or placed it where 32 bits cannot reach.
    pub fn map(device_id: u64, epoch: u64, bytes: usize) -> Option<Grant> {
        let len = (bytes.checked_add(0xfff)? & !0xfff) as u64;
        let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
        let flags = MK_DMA_MAP_COHERENT | MK_DMA_MAP_DMA32;
        if mk_dma_map(device_id, epoch, len, flags, &mut out) < 0 || out.length < len {
            return None;
        }
        (out.device_addr.checked_add(out.length)? <= 1 << 32).then_some(Grant {
            va: out.user_va as usize,
            dev: out.device_addr,
            len: out.length as usize,
        })
    }

    /// `len` bytes from `off`, the same memory under the same grant.
    pub fn sub(&self, off: usize, len: usize) -> Option<Grant> {
        (off.checked_add(len)? <= self.len).then_some(Grant {
            va: self.va + off,
            dev: self.dev + off as u64,
            len,
        })
    }

    pub fn size(&self) -> usize {
        self.len
    }

    /// The address the chip reaches byte `off` at.
    pub fn dev(&self, off: usize) -> u32 {
        (self.dev + off as u64) as u32
    }

    pub(super) fn inside(&self, off: usize, n: usize) -> bool {
        off.checked_add(n).is_some_and(|end| end <= self.len)
    }
}
