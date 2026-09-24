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

//! How much of the store each kind of node may take.
//!
//! The three kinds share one store, so they are budgeted against each other:
//! left unbudgeted the first list read fills it and the ones after are
//! dropped, which is how a directory ends up with mix layers and no gateway
//! to leave by.
//!
//! Each budget covers the whole active set for its role with room to grow.
//! The endpoints answer with the active set rather than every bonded node, so
//! taking all of it is correct rather than greedy: a node outside the active
//! set is not one a route may use. Truncating instead would leave every
//! client holding the same prefix and entering the mixnet through the same
//! few gateways, which costs anonymity and saves nothing. The whole set is
//! under 40 KB.

/// Mix hops. The active set is three layers of twenty. Budgeted well above
/// that so a layer widening does not silently start dropping hops.
pub const MIX_BUDGET: usize = 128;

/// Gateways to hold a session with. The active set is around 180.
pub const ENTRY_BUDGET: usize = 192;

/// Gateways to leave by. Around 179 active, and kept equal to entry so
/// neither starves the other.
pub const EXIT_BUDGET: usize = 192;

// The store refuses a list longer than it can hold, and it refuses the whole
// list rather than the tail, so a budget that overruns loses everything
// rather than the excess.
const _: () = assert!(MIX_BUDGET + ENTRY_BUDGET + EXIT_BUDGET <= crate::topology::NODE_CAP);
