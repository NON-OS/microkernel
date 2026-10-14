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

//! A bus that stops moving, and the time a transfer spends on it. A wait
//! must give the processor back while it lasts and end at the
//! transfer's deadline, however long the bus stays stopped.

use nonos_i2cmodel::Config;
use nonos_libc::{mk_uptime_ms, slept_ms, yields};

use super::fixture::{bench, core, PAD};
use crate::transaction::{transfer, TransferError, TransferRequest, FLAG_RESTART_ON_READ};

pub(super) const DESC: TransferRequest<'static> =
    TransferRequest { addr: PAD, flags: FLAG_RESTART_ON_READ, write: &[0x01, 0x00], read_len: 4 };

pub(super) fn read(b: &super::fixture::Bench) -> Result<Vec<u8>, TransferError> {
    let r = transfer(&b.driver, TransferRequest { ..DESC })?;
    Ok(r.read[..r.read_len].to_vec())
}

#[test]
fn a_transfer_on_a_stalled_bus_waits_asleep_and_ends_at_its_deadline() {
    let b = bench(Config::LPSS);
    core(|c| c.stall_bus(false));
    let (y, s, t) = (yields(), slept_ms(), mk_uptime_ms());
    assert!(matches!(read(&b), Err(TransferError::Timeout)));
    let waited = (mk_uptime_ms() - t) as u64;
    assert!(yields() > y, "the wait never yielded the processor");
    // The shim's clock moves a millisecond per read as well as per sleep,
    // so a poll that reads it and sleeps counts two; a third is the floor.
    assert!(3 * (slept_ms() - s) > waited, "slept {} of {waited} ms", slept_ms() - s);
    assert!((50..200).contains(&waited), "waited {waited} ms");
}

#[test]
fn a_healthy_transfer_never_sleeps() {
    // A sleep is a millisecond at least; a touchpad read takes about one.
    let b = bench(Config::LPSS);
    let s = slept_ms();
    assert!(read(&b).is_ok());
    assert_eq!(slept_ms(), s, "a moving bus was waited on asleep");
}

#[test]
fn a_request_with_nothing_to_move_is_refused_before_the_bus() {
    let b = bench(Config::LPSS);
    let empty = TransferRequest { addr: PAD, flags: 0, write: &[], read_len: 0 };
    assert!(matches!(transfer(&b.driver, empty), Err(TransferError::Invalid)));
    assert!(core(|c| c.bus().trace().is_empty()), "an empty transfer reached the bus");
}
