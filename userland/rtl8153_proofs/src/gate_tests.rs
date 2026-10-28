// NONOS Operating System (AGPL-3.0-or-later)
//! Receive stays gated and off until the link is up, then the gate lifts
//! and the filter is set last.

use crate::chip::{Access, PLA};
use crate::helpers::{bound, first, last, link_to, wrote, CODES, GIGA_UP};

#[test]
fn receive_stays_gated_and_off_until_the_link_is_up() {
    for code in CODES {
        let (p, mut r) = bound(code);
        assert!(first(&p, wrote(PLA, 0xe810, 0x0c00_0000)).is_none(), "{code:04x}: on while down");
        let gate = first(&p, wrote(PLA, 0xe858, 0x0008_0000)).unwrap();
        link_to(&p, &mut r, GIGA_UP).unwrap();
        let on = first(&p, wrote(PLA, 0xe810, 0x0c00_0000)).unwrap();
        let lift =
            last(&p, |a| matches!(*a, Access::Write(PLA, 0xe858, _, v) if v & 0x0008_0000 == 0));
        let groups = last(&p, |a| matches!(*a, Access::Write(PLA, 0xcd04, _, _))).unwrap();
        let filter = last(&p, wrote(PLA, 0xc010, 0x0e)).unwrap();
        assert!(
            gate < on && on < lift.unwrap() && lift.unwrap() < groups && groups < filter,
            "{code:04x}"
        );
    }
}
