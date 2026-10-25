// NONOS Operating System (AGPL-3.0-or-later)
//! TDs a hostile device or a confused controller retired: a CBP outside
//! the buffer counts no bytes, never all of them; a run that ended without
//! a halt but with a TD retired in error is the halt it should have been.

use std::cell::RefCell;

use crate::env::Mem;
use crate::facts::td::{TD_CC_SHIFT, TD_LEN};
use crate::sched::clean::clean;
use crate::sched::ed::Target;
use crate::sched::layout::BULK_TD;
use crate::sched::queue::Ran;
use crate::sched::td::{cc, moved, write, Td};
use crate::xfer::bulk::{ended, BulkEp};
use crate::xfer::failed::CtrlError;

/// A word memory at bus address 0x1000_0000 + offset.
struct Words(RefCell<Vec<u32>>);

impl Mem for Words {
    fn bus(&self, off: usize) -> u32 {
        0x1000_0000 + off as u32
    }
    fn rd(&self, off: usize) -> u32 {
        self.0.borrow()[off / 4]
    }
    fn wr(&self, off: usize, v: u32) {
        self.0.borrow_mut()[off / 4] = v;
    }
    fn read_bytes(&self, _off: usize, _out: &mut [u8]) {}
    fn write_bytes(&self, _off: usize, _src: &[u8]) {}
}

fn words() -> Words {
    Words(RefCell::new(vec![0; 1024]))
}

const BUF: usize = 0x400;

fn retired(cbp: u32, cc: u32) -> Words {
    let m = words();
    write(&m, 0, Td { dp: 2, toggle: None, round: true, buf: BUF, len: 64 }, TD_LEN);
    m.wr(0, (m.rd(0) & 0x0FFF_FFFF) | cc << TD_CC_SHIFT);
    m.wr(4, cbp);
    m
}

#[test]
fn a_cbp_outside_the_buffer_counts_no_bytes() {
    let base = 0x1000_0000 + BUF as u32;
    assert_eq!(moved(&retired(0, 0), 0, BUF, 64), 64, "all moved");
    assert_eq!(moved(&retired(base + 10, 0), 0, BUF, 64), 10, "short");
    assert_eq!(moved(&retired(base - 4, 0), 0, BUF, 64), 0, "below the buffer");
    assert_eq!(moved(&retired(base + 64, 0), 0, BUF, 64), 0, "past its end");
}

#[test]
fn a_td_retired_in_error_without_a_halt_is_a_halt() {
    assert_eq!(clean(&retired(0, 0), Ran::Done, 0, 1), Ran::Done);
    // 8 is DataOverrun, the babble code (OHCI 1.0a 4.3.3, as recalled).
    let overrun = retired(0, 8);
    assert_eq!(cc(&overrun, 0), 8);
    assert_eq!(clean(&overrun, Ran::Done, 0, 1), Ran::Halted);
    assert_eq!(clean(&overrun, Ran::TimedOut, 0, 1), Ran::TimedOut);
    let m = words();
    m.wr(BULK_TD, 8 << TD_CC_SHIFT);
    let t = Target { addr: 1, ep: 2, low_speed: false };
    let mut ep = BulkEp { t, mps: 64, index: 0, at: 0 };
    assert_eq!(ended(&m, &mut ep, Ran::Done, BUF, 64), Err(CtrlError::Bus(8)));
}
