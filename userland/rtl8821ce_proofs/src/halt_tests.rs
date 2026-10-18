// NONOS Operating System (AGPL-3.0-or-later)
//! A download that does not finish (a bad checksum, an 8051 never ready, a
//! section never staged or a DDMA channel that never let go) leaves the 8051
//! halted, its clock off and the WL platform held in reset.

use core::cell::RefCell;

use crate::clock::now_ms;
use crate::fw::prep::{abort, begin, finish, Finish};
use crate::fw::regs::{
    CHECK_SUM_OK, CPU_CLK_EN_HI, FEN_CPUEN_HI, MCUFWDL_EN, REG_CPU_DMEM_CON, REG_MCUFW_CTRL,
    REG_RSV_CTRL, REG_SYS_CLK_CTRL, REG_SYS_FUNC_EN, WLMCU_IOIF_HI, WL_PLATFORM_RST_B2,
};
use crate::regs::Mmio;

/// A register file whose every bit starts set, so each one the halt must
/// clear is seen cleared.
struct Card(RefCell<Vec<u8>>);

impl Mmio for Card {
    fn read8(&self, o: usize) -> u8 {
        self.0.borrow()[o]
    }
    fn write8(&self, o: usize, v: u8) {
        self.0.borrow_mut()[o] = v;
    }
    fn read16(&self, o: usize) -> u16 {
        u16::from_le_bytes([self.read8(o), self.read8(o + 1)])
    }
    fn write16(&self, o: usize, v: u16) {
        v.to_le_bytes().iter().enumerate().for_each(|(i, b)| self.write8(o + i, *b));
    }
    fn read32(&self, o: usize) -> u32 {
        u32::from_le_bytes([self.read8(o), self.read8(o + 1), self.read8(o + 2), self.read8(o + 3)])
    }
    fn write32(&self, o: usize, v: u32) {
        v.to_le_bytes().iter().enumerate().for_each(|(i, b)| self.write8(o + i, *b));
    }
}

fn halted(c: &Card) -> bool {
    c.read8(REG_SYS_FUNC_EN + 1) & FEN_CPUEN_HI == 0
        && c.read8(REG_RSV_CTRL + 1) & WLMCU_IOIF_HI == 0
        && c.read16(REG_MCUFW_CTRL) & MCUFWDL_EN == 0
        && c.read8(REG_SYS_CLK_CTRL + 1) & CPU_CLK_EN_HI == 0
        && c.read8(REG_CPU_DMEM_CON + 2) & WL_PLATFORM_RST_B2 == 0
}

#[test]
fn a_bad_checksum_leaves_the_8051_halted() {
    let c = Card(RefCell::new(vec![0xFF; 0x2000]));
    let backup = begin(&c);
    c.write16(REG_MCUFW_CTRL, !CHECK_SUM_OK);
    assert!(matches!(finish(&c, backup), Finish::ChecksumBad(_)));
    assert!(halted(&c));
}

#[test]
fn an_8051_never_ready_is_halted_after_its_time() {
    let c = Card(RefCell::new(vec![0xFF; 0x2000]));
    let backup = begin(&c);
    let start = now_ms();
    assert!(matches!(finish(&c, backup), Finish::NotReady(_)), "all ones is not FW_READY");
    assert!(now_ms() - start >= 200);
    assert!(halted(&c));
}

#[test]
fn an_aborted_download_leaves_the_8051_halted() {
    let c = Card(RefCell::new(vec![0xFF; 0x2000]));
    let backup = begin(&c);
    abort(&c, backup);
    assert!(halted(&c));
}
