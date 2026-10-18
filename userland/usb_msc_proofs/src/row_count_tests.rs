// NONOS Operating System (AGPL-3.0-or-later)
//! What starts the count of failed requests a device is kept after again
//! (scan/keep.rs, state/held.rs): a READ or a WRITE that moves the kernel's
//! data. A flush moves none, and a stick that fails every READ may still
//! take one; had it counted, such a stick was kept for the boot.

use crate::answers::{fails, passes_check, OK};
use crate::kernel::{flush, write};
use crate::protocol::{E_AGAIN, E_IO};
use crate::scan::report::Stage;
use crate::scan::Medium;
use crate::server::serve::Server;
use crate::serving::read;
use crate::sticks::bound;
use crate::xhci::{setup, Answer};

#[test]
fn a_read_or_write_that_moves_data_starts_the_count_again_and_a_flush_does_not() {
    let mut server = bound(1000);
    let fail = |server: &mut Server| {
        setup(|d| d.answers = fails(0x03, 0x11).into_iter().chain(passes_check(1000)).collect());
        assert_eq!(read(server), E_IO);
        matches!(server.scanner.medium, Medium::Ready(_))
    };
    let moved = [Answer::Data(vec![0; 512]), OK];
    assert_eq!([fail(&mut server), fail(&mut server)], [true; 2]);
    setup(|d| d.answers = moved.into_iter().collect());
    assert_eq!(read(&mut server), 0);
    assert_eq!([fail(&mut server), fail(&mut server)], [true; 2]);
    assert_eq!(write(&mut server), 0);
    assert_eq!([fail(&mut server), fail(&mut server)], [true; 2]);
    assert_eq!(flush(&mut server), 0);
    assert!(!fail(&mut server), "the third since the write lets it go");
    assert_eq!(server.scanner.report.stage, Stage::NotReady);
}

/// A medium served afresh is kept too, and counts: a stick that says a
/// power-on under every READ before the kernel holds a sector, each answered
/// E_AGAIN, would have the kernel ask again for the boot.
#[test]
fn a_medium_served_afresh_counts_so_the_kernel_is_not_asked_again_for_ever() {
    let mut server = bound(1000);
    for kept in [true, true, false] {
        setup(|d| d.answers = fails(0x06, 0x29).into_iter().chain(passes_check(1000)).collect());
        assert_eq!(read(&mut server), E_AGAIN, "nothing held: asked to come again");
        assert_eq!(matches!(server.scanner.medium, Medium::Ready(_)), kept);
    }
    assert_eq!(server.scanner.report.stage, Stage::NotReady);
}
