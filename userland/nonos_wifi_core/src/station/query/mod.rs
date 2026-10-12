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

//! The station's side of an SA Query (IEEE Std 802.11-2020, 11.13): a request
//! every `RETRY_MS`, each with a new transaction identifier, until a protected
//! response names one of them, or `MAX_MS` passes and the association counts
//! as gone. Only one runs at a time.

mod due;
mod state;

pub(super) use due::Asked;
pub(super) use state::Query;
pub use state::{MAX_MS, RETRY_MS};
