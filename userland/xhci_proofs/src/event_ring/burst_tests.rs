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

//! A SuperSpeed bulk endpoint's burst reaches its endpoint context.

use super::fixture::{Fixture, BULK_IN_DCI, BULK_OUT_DCI};
use super::toggle_slot::{dw, storage_slot, CONTEXT_SIZE};
use crate::controller::{write_bulk_input, BulkEndpoint};

#[test]
fn a_superspeed_burst_is_written_into_the_endpoint_context() {
    let fx = Fixture::new();
    let res = storage_slot(&fx);
    let ep =
        |dci, burst| BulkEndpoint { dci, ring_phys: 0x10_0000, max_packet: 1024, max_burst: burst };
    write_bulk_input(
        &res.input_context,
        &res.output_context,
        CONTEXT_SIZE,
        ep(BULK_IN_DCI, 15),
        ep(BULK_OUT_DCI, 3),
    );
    let burst = |dci: u8| (dw(&res.input_context, dci as usize + 1, 1) >> 8) & 0xFF;
    assert_eq!(burst(BULK_IN_DCI), 15);
    assert_eq!(burst(BULK_OUT_DCI), 3);
    write_bulk_input(
        &res.input_context,
        &res.output_context,
        CONTEXT_SIZE,
        ep(BULK_IN_DCI, 0x1F),
        ep(BULK_OUT_DCI, 0),
    );
    assert_eq!(burst(BULK_IN_DCI), 15, "bMaxBurst is four bits");
    assert_eq!(burst(BULK_OUT_DCI), 0);
}
