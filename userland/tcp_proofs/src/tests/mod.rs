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

//! The proofs, one file per behaviour.

mod ack_range;
mod broadcast;
mod close_wait;
mod fin_order;
mod half_open;
mod handshake;
mod hostile;
mod linger;
mod mss;
mod orphans;
mod overlap;
mod passive_open;
mod persist;
mod recv_bound;
mod refusal;
mod rst;
mod rtt;
mod window_update;
