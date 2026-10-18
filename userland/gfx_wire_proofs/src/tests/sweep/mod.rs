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

//! Seeded sweeps over the display wire both ways: every decoder held to
//! the test's own reading of the frame, honest values round-tripped, and
//! each lie (a length, a count, a field past its bound) refused.

mod body_lies;
mod driver_bodies;
mod fit_sweep;
mod frames;
mod modes_lies;
mod reply_round_trip;
mod reply_sweep;
mod request_sweep;
mod rng;
mod round_trip;
