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

/* Pieces consumed only by the host proofs until their callers land: the
 * element state restyle decision waits on the selector engine's element
 * state, and the rest are the inner parts the proofs check one by one. */
pub use super::blit::Shift;
pub use super::classify_host::{host_kind, HostKind};
pub use super::damage_map::damage_for;
pub use super::history::HISTORY_CAP;
pub use super::query_encode::encode_query;
pub use super::restyle::{state_restyle, Restyle, ACTIVE, FOCUS, HOVER};
pub use super::scroll::{max_scroll, page_step, LINE_PX, WHEEL_PX};
