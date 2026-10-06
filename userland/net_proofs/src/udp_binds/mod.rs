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

//! net.udp's bind table, from capsule source.

#[allow(dead_code)]
#[path = "../../../capsule_net_udp/src/state/bind.rs"]
pub mod bind;
// The real code's own style, allowed on the include rather than restyled here.
#[allow(dead_code, clippy::new_without_default)]
#[path = "../../../capsule_net_udp/src/state/table.rs"]
pub mod table;
