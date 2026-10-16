// NONOS Operating System (AGPL-3.0-or-later)
//! A START UNIT that fails has its sense taken, as any command's
//! (disk/ready.rs). It was never asked: a 28h the unit failed it for, said
//! only once, was lost, and a held medium swapped under it was let go and
//! bound again where a change said anywhere else ends serving.

use crate::answers::fails;
use crate::fixture::ready_unit;
use crate::kernel::generation;
use crate::protocol::{E_IO, E_NODEV};
use crate::scan::report::Stage;
use crate::serving::{read, until_ready};
use crate::sticks::held;
use crate::xhci::{setup, Answer};

/// A READ failed with a medium error, then a check whose TEST UNIT READY
/// says NOT READY, becoming ready, and whose START UNIT fails with `key`
/// and `asc`.
fn start_fails(key: u8, asc: u8) -> Vec<Answer> {
    let check = fails(0x02, 0x04).into_iter().chain(fails(key, asc));
    fails(0x03, 0x11).into_iter().chain(check).collect()
}

#[test]
fn a_change_said_under_a_start_unit_in_the_check_of_a_held_medium_ends_serving() {
    let mut server = held(1000);
    setup(|d| d.answers = start_fails(0x06, 0x28).into());
    assert_eq!(read(&mut server), E_IO);
    assert_eq!(server.scanner.report.stage, Stage::Changed, "the 28h was heard");
    assert_eq!(read(&mut server), E_NODEV);
}

#[test]
fn a_start_unit_failed_in_the_check_lets_go_and_the_bind_after_is_told() {
    let mut server = held(1000);
    let answers = start_fails(0x02, 0x04).into_iter().chain(ready_unit(1000));
    setup(|d| d.answers = answers.collect());
    assert_eq!(read(&mut server), E_IO);
    assert_eq!(server.scanner.report.stage, Stage::NotReady, "a failed check lets it go");
    until_ready(&mut server, 2_000);
    assert_eq!(generation(&mut server), 1, "bound again and told");
}
