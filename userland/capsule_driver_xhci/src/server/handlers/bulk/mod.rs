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
//! Bulk endpoints for mass-storage, network and Bluetooth class drivers:
//! configure them, move data through them, poll the IN, and recover one
//! that stalled.
mod configure;
mod configure_pipes;
mod max_packet;
mod poll;
pub(crate) mod reply;
mod reset;
mod run;
mod streams;
mod streams_give;
mod timeout;
mod transfer;
pub use configure::{configure, configure_second};
pub use poll::poll_in;
pub use reset::reset;
pub use streams::configure_streams;
pub use transfer::{transfer_in, transfer_out};
