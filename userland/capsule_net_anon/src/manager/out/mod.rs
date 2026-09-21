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

//! Everything this capsule sends on a circuit.

mod buffered;
mod open_stream;
mod pick_circuit;
mod send_data;
mod send_end;
mod send_relay;
mod sendme_tick;
mod stream_sendme;
mod window;

pub use open_stream::open_stream;
pub use send_data::send_data;
pub use send_end::send_end;
pub use send_relay::SendError;
pub use sendme_tick::tick as sendme_tick;
