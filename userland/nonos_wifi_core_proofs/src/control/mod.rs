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

//! E-62: the one control entry the shell reaches the driver through, swept.
//! Over seeded requests — any magic, any op, any length — `handle` never
//! panics and its reply fits the buffer, a known WIFI op echoes its header,
//! and anything else is refused cleanly (`sweep`); and no reply to a joined
//! station ever carries its passphrase (`leak`).

mod leak;
mod sweep;
mod wire;
