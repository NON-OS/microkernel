// NONOS Operating System (AGPL-3.0-or-later)
//! The version read retried: three tries, and no more.

use nonos_usbnet::Bind;

use crate::chip::plug;
use crate::helpers::try_bind;

#[test]
fn the_version_read_is_tried_three_times_then_given_up() {
    let p = plug(0x5c30);
    p.chip.borrow_mut().fail_after = Some(0);
    let Bind::Failed(what, e) = try_bind(&p) else { panic!("bound with no version") };
    assert_eq!((what, e), ("version register unread", -71));
    assert_eq!(p.chip.borrow().transfers, 3);
}

#[test]
fn a_version_read_that_fails_twice_is_read_a_third_time() {
    let p = plug(0x6010);
    p.chip.borrow_mut().flaky = 2;
    assert!(matches!(try_bind(&p), Bind::Ours(_)), "two failed reads gave the chip up");
    let p = plug(0x6010);
    p.chip.borrow_mut().flaky = 3;
    assert!(matches!(try_bind(&p), Bind::Failed("version register unread", -71)));
}
