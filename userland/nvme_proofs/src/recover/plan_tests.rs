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

//! The recovery plan over a scripted controller: whatever CSTS reads, it
//! ends in a named state, aborts only a controller that reads ready and not
//! fatal, waits out the grace only for an Abort the controller says it
//! carried out, otherwise looks at the ring once, and resets whatever that
//! does not settle.

use super::script::{run, REFUSED, TIMEOUT};
use super::Recovered;
use crate::error::NvmeError;

#[test]
fn a_ready_controller_is_asked_to_abort_before_any_reset() {
    // RDY alone, or with shutdown status and NSSR bits: an Abort goes out.
    for csts in [0x1, 0x1 | 0x4, 0x1 | 0x8 | 0x10] {
        let (end, did) = run(csts, Ok(true), [Ok(()), Ok(())]);
        assert_eq!((end, did), (Recovered::Aborted, vec!["csts", "abort", "grace"]));
        let (end, did) = run(csts, Ok(true), [TIMEOUT, Ok(())]);
        assert_eq!((end, did), (Recovered::Reset, vec!["csts", "abort", "grace", "reset"]));
        // Not aborted (DW0 bit 0), refused, or no answer (the admin queue is
        // stuck too): no grace, one look at the ring for a command that
        // completed before the Abort reached it, then the reset.
        let not_aborted = [
            Ok(false),
            Err(NvmeError::AdminCommandFailed),
            Err(NvmeError::ControllerTimeout),
            Err(NvmeError::ClockFailed),
        ];
        for abort in not_aborted {
            let (end, did) = run(csts, abort, [TIMEOUT, Ok(())]);
            let looked = vec!["csts", "abort", "look", "reset"];
            assert_eq!((end, did), (Recovered::Reset, looked), "{abort:?}");
            let (end, did) = run(csts, abort, [Ok(()), Ok(())]);
            assert_eq!((end, did), (Recovered::Aborted, vec!["csts", "abort", "look"]));
        }
    }
}

#[test]
fn a_controller_not_ready_or_fatal_is_reset_without_an_abort() {
    for csts in [0x0, 0x2, 0x3, 0x4, u32::MAX - 1, 0x7fff_ffff, 0x8000_0003] {
        let (end, did) = run(csts, Ok(true), [Ok(()), Ok(())]);
        assert_eq!((end, did), (Recovered::Reset, vec!["csts", "reset"]), "csts {csts:#x}");
    }
}

#[test]
fn all_ones_is_a_controller_gone_and_nothing_is_tried() {
    let (end, did) = run(u32::MAX, Ok(true), [Ok(()), Ok(())]);
    assert_eq!((end, did), (Recovered::Gone, vec!["csts"]));
}

#[test]
fn a_reset_that_fails_ends_failed_whatever_came_before() {
    for e in [TIMEOUT, REFUSED, Err(NvmeError::ControllerFatal), Err(NvmeError::ClockFailed)] {
        for (csts, abort) in [(0x1, Ok(true)), (0x1, Ok(false)), (0x3, Ok(true))] {
            assert_eq!(run(csts, abort, [TIMEOUT, e]).0, Recovered::Failed, "csts {csts:#x}");
        }
    }
}
