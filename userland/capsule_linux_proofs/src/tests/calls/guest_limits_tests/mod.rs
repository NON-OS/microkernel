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

//! What a guest can make this capsule hold: tables of pipes, counters,
//! timers and signalfd masks that grow only as far as the guest holds
//! descriptors, control data that is walked to its end and no further
//! whatever lengths it claims, a resolver that takes and keeps what a
//! datagram socket would, and a display stream that a malformed header
//! cannot stall.

mod held;
mod random;
mod rights;
mod slots;
