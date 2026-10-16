// NONOS Operating System (AGPL-3.0-or-later)
//! qTDs a hostile device or a confused controller retired: a total left
//! past what was asked counts no bytes, never a wrap; a short qTD moves
//! what it moved; a halt with babble, buffer or transaction bits fails the
//! transfer with them, and one with none is a STALL.

use std::cell::RefCell;

use crate::env::Mem;
use crate::facts::qtd::{PID_IN, TOK_ACTIVE, TOK_HALTED};
use crate::sched::qtd::{moved, write, Qtd};
use crate::sched::queue::{look, Ran};
use crate::xfer::failed::{failed, CtrlError};

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

const BABBLE: u32 = 1 << 4;

fn retired(token: u32) -> Words {
    let m = words();
    write(&m, 0, Qtd { pid: PID_IN, toggle: false, buf: 0x400, len: 64 }, None, None);
    m.wr(8, token);
    m
}

#[test]
fn a_total_left_past_the_request_counts_no_bytes() {
    assert_eq!(moved(&retired(0x7FFF << 16), 0, 64), 0, "32767 left of 64");
    assert_eq!(moved(&retired(54 << 16), 0, 64), 10, "short");
    assert_eq!(moved(&retired(0), 0, 64), 64);
}

#[test]
fn a_halt_fails_with_its_bits_and_an_active_qtd_is_not_done() {
    assert_eq!(look(&retired(TOK_ACTIVE), &[0]), None);
    let babble = retired(TOK_HALTED | BABBLE);
    assert_eq!(look(&babble, &[0]), Some(Ran::Halted(TOK_HALTED | BABBLE)));
    assert_eq!(failed(TOK_HALTED | BABBLE), CtrlError::Bus(BABBLE));
    assert_eq!(failed(TOK_HALTED), CtrlError::Stalled);
}
