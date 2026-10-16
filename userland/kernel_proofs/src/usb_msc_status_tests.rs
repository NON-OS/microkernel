// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The status of driver.usb_msc0's reply as the kernel's client reads it
//! (src/hardware/usb_msc_capsule/status.rs), driven by the driver's own
//! errnos, so the two ends of the wire are held together. The block layer's
//! disk selection asks a refusal again, and passes a device failure over
//! (src/hardware/block_device/identify.rs).

#[path = "../../capsule_driver_usb_msc/src/protocol/errno.rs"]
mod driver;
#[path = "../../../src/hardware/usb_msc_capsule/error.rs"]
mod error;
#[path = "../../../src/hardware/usb_msc_capsule/status.rs"]
mod status;

use driver::*;
use error::DriverUsbMscError as E;
use status::lift;

#[test]
fn every_errno_the_driver_answers_with_is_read_as_it_means() {
    assert_eq!(lift(0), Ok(()));
    for (errno, read) in [
        (E_AGAIN, E::NotReady),
        (E_TIMEDOUT, E::NotReady),
        (E_NODEV, E::NoDevice),
        (E_NOMEDIUM, E::NoDevice),
        (E_ACCES, E::AccessDenied),
        (E_INVAL, E::InvalidArgument),
        (E_NXIO, E::OutOfRange),
        (E_MSGSIZE, E::OversizedRequest),
        (E_NOTSUP, E::BlockSize),
        (E_IO, E::DeviceFailure),
        (E_BAD_OP, E::DeviceFailure),
        (E_NO_MSC, E::DeviceFailure),
        (E_OVERFLOW, E::DeviceFailure),
        (E_PHASE, E::DeviceFailure),
    ] {
        assert_eq!(lift(errno), Err(read), "errno {errno}");
    }
}

#[test]
fn a_request_out_of_its_time_is_a_refusal_the_disk_selection_asks_again() {
    assert_eq!(lift(E_TIMEDOUT), Err(E::NotReady), "one 18 s timeout passed a live stick over");
}

#[test]
fn no_other_status_reads_as_success_or_as_the_transports_own_failure() {
    let transport = [E::Dead, E::Stale, E::NoCallerPid, E::TransportFailure, E::ProtocolMismatch];
    for status in (-200..=200).chain([i32::MIN, i32::MAX]).filter(|&s| s != 0) {
        let read = lift(status);
        assert!(read.is_err(), "status {status}");
        assert!(transport.iter().all(|&t| read != Err(t)), "status {status}: {read:?}");
    }
}
