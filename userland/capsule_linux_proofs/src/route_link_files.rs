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

/*
 * The network a guest's stream leaves through is nonos_route_link's pick, as
 * it ships, and a stream's bound, its refusals and the frames that carry it
 * to net.socks5 are its own too. At the crate root, where their own `crate::`
 * paths look for them.
 */
#[path = "../../nonos_route_link/src/bounds.rs"]
pub mod bounds;
#[path = "../../nonos_route_link/src/frame.rs"]
pub mod frame;
#[path = "../../nonos_route_link/src/frame_on.rs"]
pub mod frame_on;
#[path = "../../nonos_route_link/src/pick.rs"]
pub mod pick;
#[path = "../../nonos_route_link/src/refusal.rs"]
pub mod refusal;
#[path = "../../nonos_route_link/src/socks.rs"]
pub mod socks;
