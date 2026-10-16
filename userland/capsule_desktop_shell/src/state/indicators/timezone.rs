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

//! The Timezone setting, read on a tick the store answered.

// Whole hours east of UTC; needs the port `clock_24h` found this tick.
pub fn timezone(port: u32) -> Option<i8> {
    (port != 0).then(|| nonos_policy_client::get_i8(port, nonos_policy_proto::Field::Timezone))?
}
