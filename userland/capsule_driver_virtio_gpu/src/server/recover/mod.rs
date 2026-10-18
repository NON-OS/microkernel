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

//! A control queue the device stalled on, brought back in place. The server runs only once the primary surface is shared, and the
//! compositor draws into its frames from then on: giving the device back
//! would have the kernel free them under it, and nothing restarts a driver.
//! So the claim and every grant stay. A stall gets at most three tries, each
//! logged (`tries`, `restore`). If none works, or the device stalled again
//! after IN_A_ROW recoveries without serving SERVED_MS after any of them
//! (`policy`), it is left reset and the driver stays up, `Down`: device ops
//! answer E_DEVICE and OP_HEALTHCHECK names the state.

pub mod policy;
mod restore;
mod tries;

pub use policy::settle;
