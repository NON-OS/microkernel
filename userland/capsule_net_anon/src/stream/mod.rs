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

//! Streams over a circuit.

mod begin;
mod connected;
mod data;
mod end;
mod grant;
mod ids;
mod sendme;
mod stage;
pub(crate) mod table;

pub use begin::body as begin_body;
pub use connected::is_valid as connected_is_valid;
pub use data::pieces;
pub use end::{is_clean, needs_another_exit, reason, REASON_DONE};
pub use ids::next as next_id;
pub use sendme::body as sendme_body;
pub use stage::StreamStage;
pub use table::Stream;
