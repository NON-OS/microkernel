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

//! The driver's side of the handshake with ASF firmware: the signatures
//! around a reset, pausing it before one, and the heartbeat that keeps it
//! from resetting the part under a live driver (tg3_stop_fw,
//! tg3_write_sig_*, tg3_timer). Without ASF only the pre-reset magic word
//! is written, as Linux writes it on every part.

mod event;
mod sig;

pub use event::{heartbeat, stop};
pub use sig::{sig_legacy, sig_post_reset, sig_pre_reset, Kind};
