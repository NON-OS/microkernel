// NONOS Operating System (AGPL-3.0-or-later)
//! TDs a hostile device or a confused controller retired: an ActLen past
//! what a TD asked for counts only what it asked; a short TD ends the run
//! short; a babble or a STALL fails it; only the TDs queued are read.

use std::cell::RefCell;

use crate::env::Mem;
use crate::facts::td::{STATUS_BABBLE, STATUS_STALLED, TD_LEN};
use crate::sched::td::Target;
use crate::xfer::chain::{moved, write, Step};
use crate::xfer::run::{look, Ran};

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

const T: Target = Target { addr: 3, ep: 1, low_speed: false };

/// Two 64-byte IN TDs at 0, then their status words set to `ctrl`.
fn ran(ctrl: [u32; 2]) -> Words {
    let m = words();
    let step = Step { pid: 0x69, toggle: false, buf: 0x400, len: 64, spd: true };
    write(&m, 0, T, &[step, Step { buf: 0x440, ..step }]);
    for (i, c) in ctrl.into_iter().enumerate() {
        m.wr(i * TD_LEN + 4, c);
    }
    m
}

#[test]
fn an_actlen_past_what_was_asked_counts_only_what_was_asked() {
    let m = ran([0x7FE, 63]);
    assert_eq!(moved(&m, 0, 0, 1), 128, "2047 claimed, 64 asked");
}

#[test]
fn a_short_td_ends_the_run_short_and_errors_fail_it() {
    let m = ran([9, 63]);
    assert_eq!(look(&m, 0, 2), Some(Ran::Ended { upto: 0, short: true }));
    assert_eq!(moved(&m, 0, 0, 0), 10);
    assert_eq!(look(&ran([STATUS_BABBLE | 63, 63]), 0, 2), Some(Ran::Error(STATUS_BABBLE)));
    assert_eq!(look(&ran([STATUS_STALLED | 0x7FF, 63]), 0, 2), Some(Ran::Stalled));
}
