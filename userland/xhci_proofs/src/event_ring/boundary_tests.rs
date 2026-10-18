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

//! The named edges of an event TRB, each put through every wait it can
//! reach: slot 0 and 255, endpoint 0 and 31, and a residual of u32::MAX
//! read as 24 bits. Pointers are `boundary_pointer_tests`.

use super::boundary_through::{through_transfers, Seen, IGNORED};
use super::events::{transfer, CC_STALL, CC_SUCCESS};
use super::fixture::SLOT;
use crate::controller::DEVICE_DESCRIPTOR_LEN;
use crate::error::XhciError;

#[test]
fn slot_0_is_taken_by_no_transfer_wait() {
    assert_eq!(through_transfers(|p, ep| transfer(p, CC_STALL, 1, 0, ep)), IGNORED);
}

#[test]
fn slot_255_is_taken_by_no_transfer_wait() {
    assert_eq!(through_transfers(|p, ep| transfer(p, CC_STALL, 1, 255, ep)), IGNORED);
}

#[test]
fn endpoint_0_is_taken_by_no_transfer_wait() {
    assert_eq!(through_transfers(|p, _| transfer(p, CC_STALL, 1, SLOT, 0)), IGNORED);
}

#[test]
fn endpoint_31_is_taken_by_no_transfer_wait() {
    assert_eq!(through_transfers(|p, _| transfer(p, CC_STALL, 1, SLOT, 31)), IGNORED);
}

#[test]
fn a_residual_of_u32_max_is_read_as_24_bits_and_clamped_to_the_request() {
    let widest = through_transfers(|p, ep| transfer(p, CC_SUCCESS, u32::MAX, SLOT, ep));
    let taken =
        Seen { poll: Ok(Some(0)), bulk: Ok(0), control: Ok(DEVICE_DESCRIPTOR_LEN as usize) };
    assert_eq!(widest, taken);

    let all_ones = through_transfers(|p, ep| {
        let mut t = transfer(p, CC_SUCCESS, 0, SLOT, ep);
        t.d2 = u32::MAX;
        t
    });
    // Completion code 0xFF is no success: the report is that error, not a
    // report of no bytes, so the server recovers the endpoint.
    let failed = XhciError::TransferCompletionFailed(0xFF);
    assert_eq!(all_ones, Seen { poll: Err(failed), bulk: Err(failed), control: Err(failed) });
}
