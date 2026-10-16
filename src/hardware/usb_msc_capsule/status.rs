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

//! driver.usb_msc0's reply status as the client's error, its errnos
//! mirrored at `userland/capsule_driver_usb_msc/src/protocol/errno.rs`.
//! Pure, so kernel_proofs holds every errno the driver answers with.

use super::error::DriverUsbMscError as E;

const E_NXIO: i32 = -6;
const E_AGAIN: i32 = -11;
const E_ACCES: i32 = -13;
const E_NODEV: i32 = -19;
const E_INVAL: i32 = -22;
const E_MSGSIZE: i32 = -90;
const E_NOTSUP: i32 = -95;
const E_TIMEDOUT: i32 = -110;
const E_NOMEDIUM: i32 = -123;

/// Ok for status 0, else the error the driver's errno means.
pub(super) fn lift(status: i32) -> Result<(), E> {
    Err(match status {
        0 => return Ok(()),
        /*
         * A request that ran out of its time is a refusal, as E_AGAIN is: the
         * driver checks the device after it, and lets it go after three
         * failed in a row. Read as a device failure, one timeout while the
         * block layer chose its disk passed a live stick over for the boot.
         */
        E_AGAIN | E_TIMEDOUT => E::NotReady,
        /* No medium in the reader is no disk there. */
        E_NODEV | E_NOMEDIUM => E::NoDevice,
        E_ACCES => E::AccessDenied,
        E_INVAL => E::InvalidArgument,
        E_NXIO => E::OutOfRange,
        E_MSGSIZE => E::OversizedRequest,
        E_NOTSUP => E::BlockSize,
        /* E_IO, and any status the driver does not document. */
        _ => E::DeviceFailure,
    })
}
