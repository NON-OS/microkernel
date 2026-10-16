// NONOS Operating System (AGPL-3.0-or-later)
//! A stalled power-sequence poll is retried once after the BIT_PFM_WOWL pulse.

use crate::pwr::command::PwrCmd;
use crate::pwr::run_pwr_seq;
use crate::pwr_mock::Chip;

#[test]
fn a_stalled_poll_is_retried_after_the_wowl_pulse() {
    let mut c = Chip::new(0xEA);
    c.on_wowl = Some((0x0010, 0x01));
    assert!(run_pwr_seq(&c, &[PwrCmd::poll(0x0010, 0x01, 0x01), PwrCmd::end()]));
    let w = c.writes.borrow();
    assert_eq!(&w[..], &[(0x04, 0x08), (0x04, 0x00)], "value|BIT3 then value&~BIT3");
}
