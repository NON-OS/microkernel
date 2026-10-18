// NONOS Operating System (AGPL-3.0-or-later)
//! What holds the medium for the kernel (state/held.rs): a READ or a WRITE
//! that moved its data, either alone, and never a request that failed or was
//! asked to come again. Once it is held, a medium change ends serving; until
//! then the medium there is served afresh (scan/check.rs).

use crate::answers::{fails, passes_check};
use crate::fixture::ready_unit;
use crate::kernel::write;
use crate::protocol::{E_AGAIN, E_IO};
use crate::scan::report::Stage;
use crate::scan::Medium;
use crate::server::serve::Server;
use crate::serving::{read, until_ready};
use crate::sticks::{bound, plug};
use crate::xhci::setup;

#[test]
fn a_write_alone_holds_the_medium() {
    let mut server = bound(1000);
    assert_eq!(write(&mut server), 0);
    setup(|d| d.answers = fails(0x06, 0x28).into_iter().chain(passes_check(1000)).collect());
    assert_eq!(read(&mut server), E_IO);
    assert_eq!(server.scanner.report.stage, Stage::Changed);
}

#[test]
fn a_request_that_failed_or_came_too_soon_holds_nothing() {
    plug(ready_unit(1000));
    let mut server = Server::new();
    assert_eq!(read(&mut server), E_AGAIN, "still searching");
    until_ready(&mut server, 2_000);
    setup(|d| d.answers = fails(0x03, 0x11).into_iter().chain(passes_check(1000)).collect());
    assert_eq!(read(&mut server), E_IO, "a medium error");
    setup(|d| d.answers = fails(0x06, 0x28).into_iter().chain(passes_check(2000)).collect());
    assert_eq!(read(&mut server), E_AGAIN);
    let Medium::Ready(d) = server.scanner.medium else { panic!("{:?}", server.scanner.report) };
    assert_eq!(d.blocks, 2000, "served afresh: the kernel held nothing");
}
