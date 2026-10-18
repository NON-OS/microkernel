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
//! A control queue the device stalled on, brought back in place (plan/50
//! C1). The compositor draws into this driver's primaries and the outputs
//! show its surfaces, whose pages the driver holds: giving the device back
//! would have the kernel free them under the host, and nothing restarts a
//! driver that gave up. So the claim and every grant stay. A stall gets at
//! most three tries, each logged (`tries`, `restore`), each showing every
//! output again on its own surface. If none works, or the device stalled
//! again after IN_A_ROW recoveries without serving SERVED_MS after any of
//! them (`policy`), it is left reset and the driver stays up, `Down`: every
//! output answers LOST and dark, and the compositor is told LEAVING once,
//! so it drives another display.

pub mod policy;
mod restore;
mod tries;

pub use policy::settle;
