// NONOS Operating System (AGPL-3.0-or-later)
//! A machine with no USB controller for the boot: driver.xhci0 is looked up
//! at every step while the search waits for it, then every few seconds, as
//! the ports are. It was looked up every 5 ms for as long as the driver ran.

use crate::clock::at;
use crate::fixture::ready_unit;
use crate::protocol::E_NODEV;
use crate::scan::Medium;
use crate::server::serve::Server;
use crate::serving::{idle, read, until_ready};
use crate::sticks::plug;
use crate::xhci::{device, setup};

#[test]
fn with_no_controller_it_is_looked_up_every_few_seconds_once_the_search_is_over() {
    plug(ready_unit(1000));
    setup(|d| d.unregistered = true);
    let mut server = Server::new();
    while !matches!(server.scanner.medium, Medium::Absent) {
        idle(&mut server);
    }
    let (t0, before) = (at(), device(|d| d.lookups));
    while at() - t0 < 60_000 {
        assert_eq!(read(&mut server), E_NODEV);
        idle(&mut server);
    }
    let looked = device(|d| d.lookups) - before;
    assert!((11..=13).contains(&looked), "looked up {looked} times in a minute");
    setup(|d| d.unregistered = false);
    let t0 = at();
    until_ready(&mut server, 10);
    assert!(at() - t0 <= 5_100, "a controller that came late is found in {} ms", at() - t0);
}
