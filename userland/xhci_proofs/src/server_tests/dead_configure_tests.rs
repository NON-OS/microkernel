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

//! An endpoint configured as the controller fails. A Host Controller Error
//! does not halt it (xHCI 1.2 section 5.4.2), so it may have taken the
//! Configure Endpoint and the rings it names: the new rings and buffers
//! stay with the slot, as a request buffer does (`dead_buffer_tests`),
//! until the revival that resets the controller frees them. Dropped at
//! once, they went back to the kernel while the controller could reach them.

use super::dead_configure::{configured_as_it_fails, freed_by_the_revival};
use crate::protocol::{OP_ALLOC_TRANSFER_RING, OP_CONFIGURE_BULK};

#[test]
fn an_interrupt_endpoint_configured_as_the_controller_fails_is_kept_until_its_reset() {
    let (b, kept, freed) =
        configured_as_it_fails(OP_ALLOC_TRANSFER_RING, |s| vec![s, 0x81, 8, 0, 4, 0]);
    assert_eq!((freed, kept.len()), (0, 2), "its ring and report buffer kept");
    freed_by_the_revival(b, &kept);
}

#[test]
fn a_bulk_pair_configured_as_the_controller_fails_is_kept_until_its_reset() {
    let pair = |s| vec![s, 0x81, 0x02, 0, 0, 2, 0, 2];
    let (b, kept, freed) = configured_as_it_fails(OP_CONFIGURE_BULK, pair);
    assert_eq!((freed, kept.len()), (0, 3), "both rings and the page kept");
    freed_by_the_revival(b, &kept);
}
