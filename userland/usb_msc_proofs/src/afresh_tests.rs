// NONOS Operating System (AGPL-3.0-or-later)
//! What the check makes of what it heard on a held medium (scan/check.rs):
//! the strongest said wins, so a 28h stays a 28h whatever follows it, and
//! an attention at another size ends serving. A medium served afresh is
//! known by the size it has now, so it is served again after a let-go.

use crate::answers::{fails, passes_check, OK};
use crate::fixture::ready_unit;
use crate::kernel::generation;
use crate::protocol::{E_AGAIN, E_IO, E_NODEV};
use crate::scan::report::Stage;
use crate::scan::Medium;
use crate::serving::{read, until_ready};
use crate::sticks::{bound, held};
use crate::xhci::setup;

#[test]
fn a_change_then_a_power_on_heard_by_the_check_of_a_held_medium_ends_serving() {
    let mut server = held(1000);
    let check = fails(0x06, 0x28).into_iter().chain(fails(0x06, 0x29)).chain(passes_check(1000));
    setup(|d| d.answers = fails(0x03, 0x11).into_iter().chain(check).collect());
    assert_eq!(read(&mut server), E_IO);
    assert_eq!(server.scanner.report.stage, Stage::Changed, "the 28h was not forgotten");
    assert_eq!(read(&mut server), E_NODEV);
}

#[test]
fn a_held_attention_at_another_size_ends_serving() {
    let mut server = held(1000);
    setup(|d| d.answers = fails(0x06, 0x29).into_iter().chain(passes_check(1001)).collect());
    assert_eq!(read(&mut server), E_IO);
    assert_eq!(server.scanner.report.stage, Stage::Changed);
    assert_eq!(read(&mut server), E_NODEV);
}

#[test]
fn a_medium_served_afresh_at_a_new_size_is_bound_again_at_it_after_a_let_go() {
    let mut server = bound(1000);
    setup(|d| d.answers = fails(0x06, 0x28).into_iter().chain(passes_check(2000)).collect());
    assert_eq!(read(&mut server), E_AGAIN);
    let not_ready = fails(0x02, 0x04).into_iter().chain([OK]).chain(ready_unit(2000));
    setup(|d| d.answers = fails(0x03, 0x11).into_iter().chain(not_ready).collect());
    assert_eq!(read(&mut server), E_IO);
    until_ready(&mut server, 2_000);
    assert!(matches!(server.scanner.medium, Medium::Ready(d) if d.blocks == 2000));
    assert_eq!(generation(&mut server), 2, "served afresh, then bound again");
}
