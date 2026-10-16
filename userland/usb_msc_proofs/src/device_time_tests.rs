// NONOS Operating System (AGPL-3.0-or-later)
//! While a device is asked the kernel's disk selection is refused (E_AGAIN),
//! so the asking of one device is held to about thirty seconds, however many
//! logical units it has and however it fails. Each unit was given its own
//! tries and thirty seconds: minutes for a sixteen-slot card reader.

use crate::clock::at;
use crate::protocol::{E_AGAIN, E_NODEV, E_TIMEDOUT};
use crate::server::serve::Server;
use crate::serving::{idle, read};
use crate::sticks::plug;
use crate::xhci::{device, setup};

/// The kernel's wait for a reply (services/lifecycle/reply_wait.rs).
const KERNEL_WAIT_MS: u64 = 35_000;

/// Requests and steps until the device's slot is given back: how long after
/// `t0` requests were still answered E_AGAIN.
fn refused_until(server: &mut Server, t0: u64) -> u64 {
    let mut last_again = 0;
    while device(|d| d.disabled.is_empty()) {
        if read(server) == E_AGAIN {
            last_again = at() - t0;
        }
        idle(server);
    }
    assert_eq!(read(server), E_NODEV, "then the kernel asks the next disk");
    last_again
}

/// A sixteen-slot reader that goes silent right after it is configured.
#[test]
fn a_silent_sixteen_slot_reader_is_given_up_within_the_kernels_wait() {
    plug(vec![]);
    setup(|d| d.max_lun = 15);
    let mut server = Server::new();
    let t0 = at();
    idle(&mut server);
    setup(|d| d.silent_ms = Some(600_000));
    let last_again = refused_until(&mut server, t0);
    assert!(last_again < KERNEL_WAIT_MS, "selection refused for {last_again} ms");
    assert_eq!(server.scanner.report.detail, E_TIMEDOUT);
}

/// A sixteen-slot reader whose every slot says NOT READY, becoming ready,
/// for ever.
#[test]
fn a_sixteen_slot_reader_never_ready_is_given_up_within_the_kernels_wait() {
    plug(vec![]);
    setup(|d| (d.max_lun, d.not_ready) = (15, true));
    let mut server = Server::new();
    let t0 = at();
    let last_again = refused_until(&mut server, t0);
    assert!(last_again < KERNEL_WAIT_MS, "selection refused for {last_again} ms");
    let luns = device(|d| d.commands.iter().filter(|&&op| op == 0x12).count());
    assert_eq!(luns, 1, "its first slot had the device's whole time");
}
