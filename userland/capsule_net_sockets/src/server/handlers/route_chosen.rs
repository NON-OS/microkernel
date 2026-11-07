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

//! The person's chosen network, as connect by name and listen ask it.

use nonos_policy_proto::Field;

use super::connect::is_direct;

/// Whether the chosen network is Direct, asked afresh each time
/// (`is_direct` holds the rule).
pub fn direct_chosen() -> bool {
    is_direct(
        nonos_policy_client::lookup()
            .and_then(|port| nonos_policy_client::get_u8(port, Field::NetworkRoute)),
    )
}
