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
//! Joining from the serving loop. A connect leaves any network first, maps
//! the transmit region on first use (one broker grant, kept while the
//! capsule lives once the firmware holds its addresses), hunts the network's
//! beacon, draws the join's randomness and runs the join
//! (`firmware::gen3::join`); the background scan rests while joined. Between
//! requests the loop services the open port: what the firmware received is
//! taken (the access point's rekeys answered and their keys installed), and
//! an association the access point ended, or a firmware that failed, closes
//! the port and takes the contexts down. net_core's link protocol is answered
//! from the open port, or as a link that is down.
mod answers;
mod ask;
mod attempt;
mod connect;
mod leave;
mod ready;
mod restart;
mod say;
mod say_link;
mod service;
mod state;
mod transmit;

pub use state::Join;
