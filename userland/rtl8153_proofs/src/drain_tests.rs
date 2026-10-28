// NONOS Operating System (AGPL-3.0-or-later)
//! A link down whose FIFOs never drain still ends each wait.

use crate::chip::{Access, Stuck, PLA};
use crate::glue::paused;
use crate::helpers::{link_to, up};
use crate::rtl::bounds::DRAIN;

fn looks(p: &crate::chip::Plugged, a: Access) -> usize {
    p.chip.borrow().log.iter().filter(|x| **x == a).count()
}

#[test]
fn a_link_down_whose_fifos_never_drain_ends_its_waits() {
    let (p, mut r) = up(0x5c30);
    p.chip.borrow_mut().stuck = Stuck::Drain;
    p.chip.borrow_mut().log.clear();
    paused();
    assert_eq!(link_to(&p, &mut r, 0), Ok(()));
    assert_eq!(looks(&p, Access::Read(PLA, 0xe84c)), DRAIN.tries as usize);
    assert_eq!(looks(&p, Access::Read(PLA, 0xe610)), DRAIN.tries as usize);
    assert!(paused() <= 3 * DRAIN.tries as u64 * DRAIN.every_ms as u64);
}
