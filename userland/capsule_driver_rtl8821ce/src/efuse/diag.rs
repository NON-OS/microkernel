// NONOS Operating System (AGPL-3.0-or-later)

//! What a stalled efuse read left, in named fields that carry no efuse
//! content. The control register's low byte is the last efuse byte read,
//! which can be part of the factory address, so it is never kept.

use core::sync::atomic::{AtomicU32, Ordering};

use super::layout::{BANK_SEL_MASK, EF_ADDR_MASK, EF_ADDR_SHIFT};

/// The 2.5V LDO enable, bit 31 of the LDO/efuse register.
const LDO25_EN_BIT: u32 = 1 << 31;

static FLAGS: AtomicU32 = AtomicU32::new(0);
static STALLED_AT: AtomicU32 = AtomicU32::new(0);
static BANK_LDO: AtomicU32 = AtomicU32::new(0);

/// The last stalled read. All zero: none has stalled since boot.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct EfuseDiag {
    /// `EfuseDiag::ALL_ONES` and `EfuseDiag::ADDR_LATCHED`.
    pub flags: u32,
    /// The byte address the read stalled on, plus one, so zero keeps meaning
    /// that no read has stalled.
    pub stalled_at: u32,
    /// The LDO/efuse register's bank-select bits (8, 9) and LDO enable (31).
    pub bank_ldo: u32,
}

impl EfuseDiag {
    /// `flags`: the register window read back all ones.
    pub const ALL_ONES: u32 = 1 << 0;
    /// `flags`: the control register held the address written, so the window
    /// answers and the efuse controller never finished.
    pub const ADDR_LATCHED: u32 = 1 << 1;
}

/// Record a read that stalled at `addr`, the control register reading `ctl`
/// and the LDO/efuse register `ldo`.
pub(super) fn record_stall(ctl: u32, addr: usize, ldo: u32) {
    let latched = (ctl >> EF_ADDR_SHIFT) & EF_ADDR_MASK == addr as u32 & EF_ADDR_MASK;
    let all_ones = ctl == u32::MAX;
    let flags = if all_ones {
        EfuseDiag::ALL_ONES
    } else if latched {
        EfuseDiag::ADDR_LATCHED
    } else {
        0
    };
    FLAGS.store(flags, Ordering::Relaxed);
    STALLED_AT.store(addr as u32 + 1, Ordering::Relaxed);
    BANK_LDO.store(ldo & (BANK_SEL_MASK | LDO25_EN_BIT), Ordering::Relaxed);
}

/// Record a window whose every efuse byte read back 0xFF.
pub(super) fn record_all_ones() {
    FLAGS.store(EfuseDiag::ALL_ONES, Ordering::Relaxed);
    STALLED_AT.store(1, Ordering::Relaxed);
}

/// The last stalled read.
pub fn diag() -> EfuseDiag {
    EfuseDiag {
        flags: FLAGS.load(Ordering::Relaxed),
        stalled_at: STALLED_AT.load(Ordering::Relaxed),
        bank_ldo: BANK_LDO.load(Ordering::Relaxed),
    }
}
