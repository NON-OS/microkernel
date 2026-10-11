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

//! The standard receive producer ring, the receive return ring, the send
//! ring and the status block the part writes back the indices into.

pub mod layout;
pub mod ret;
pub mod rx;
pub mod rx_parse;
mod rx_post;
mod status;
pub mod tx;
mod tx_post;
mod tx_reclaim;

pub use rx::RxQueue;
pub use status::Status;
pub use tx::TxRing;
