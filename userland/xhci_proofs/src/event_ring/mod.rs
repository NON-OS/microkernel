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

//! The event ring, against a controller that writes whatever it likes.
//!
//! The controller owns every byte of an event TRB: the cycle bit, the type,
//! the completion code, the slot and endpoint ids, the residual length and
//! the 64-bit pointer. These proofs run the driver's own `EventRing` and
//! every function that consumes it (the interrupt drain, both completion
//! waits, the interrupt-IN poll and the bulk wait) over host DMA memory,
//! with a producer written from the specification on the other side.

mod events;
mod fixture;
mod producer;

mod address_tests;
mod boundary_tests;
mod dequeue_tests;
mod erdp_tests;
mod fuzz_tests;
mod match_tests;
mod recover_tests;
mod residual_tests;
mod toggle_tests;
