// NONOS Operating System (AGPL-3.0-or-later)
//! Steps that wait on a card that never answers, each ending on rtw88's
//! budget for it with its failure named.

use std::cell::Cell;

use crate::clock::now_ms;
use crate::h2c::H2c;
use crate::pcie::{hold_link_awake, DbiTimeout};
use crate::regs::Mmio;

const REG_HMETFR: usize = 0x01CC;
const REG_DBI_FLAG_BYTE2: usize = 0x03F2;
const DBI_WFLAG: u8 = 1;

/// Every mailbox full; a DBI read that completes and a write that does not.
#[derive(Default)]
struct Stuck {
    flag: Cell<u8>,
}

impl Mmio for Stuck {
    fn read8(&self, off: usize) -> u8 {
        match off {
            REG_HMETFR => 0xFF,
            REG_DBI_FLAG_BYTE2 if self.flag.get() == DBI_WFLAG => DBI_WFLAG,
            _ => 0,
        }
    }
    fn write8(&self, off: usize, v: u8) {
        if off == REG_DBI_FLAG_BYTE2 {
            self.flag.set(v);
        }
    }
    fn read16(&self, _: usize) -> u16 {
        0
    }
    fn write16(&self, _: usize, _: u16) {}
    fn read32(&self, _: usize) -> u32 {
        0
    }
    fn write32(&self, _: usize, _: u32) {}
}

/// How long `f` took on the proofs' clock, with what it gave.
fn timed<T>(f: impl FnOnce() -> T) -> (T, u64) {
    let start = now_ms();
    let got = f();
    (got, now_ms() - start)
}

#[test]
fn a_full_mailbox_fails_the_command_after_three_milliseconds() {
    let (sent, took) = timed(|| H2c::new().send(&Stuck::default(), 1, 2));
    assert!(!sent);
    assert!((3..=5).contains(&took), "{took} ms");
}

#[test]
fn a_dbi_write_that_never_clears_is_reported_after_its_millisecond() {
    let (held, took) = timed(|| hold_link_awake(&Stuck::default()));
    assert_eq!(held, Err(DbiTimeout::Write), "the read cleared, the write did not");
    assert!((1..=3).contains(&took), "{took} ms");
}
