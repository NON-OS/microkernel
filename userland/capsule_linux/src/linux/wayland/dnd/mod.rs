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

//! Drag and drop within a guest, as wl_data_device 3 has it. A drag starts
//! from a press the client was sent and takes the pointer until the button
//! is let go (start.rs); each surface it crosses is entered with an offer
//! of the source's types and told the motion (over.rs); what the client
//! under it accepts and the actions it takes choose the drop (offer.rs);
//! the release drops or cancels, and the destination reads the data as a
//! selection's is read, then finishes (end.rs). Nothing is dragged between
//! guests or out of one. The drag's icon is the client's surface, not shown
//! here. The pure parts, which the host proofs mount, are action.rs and
//! state.rs.

pub mod action;
pub mod end;
pub mod offer;
pub mod over;
pub mod start;
pub mod state;

pub use state::Dnd;
