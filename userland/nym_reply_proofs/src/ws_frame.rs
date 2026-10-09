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

//! The websocket framing a gateway pushes replies in, the parts that stand
//! alone: reading one frame out of the bytes held.

#[path = "../../capsule_net_nym/src/gateway_client/ws/frame/parse.rs"]
pub mod parse;
#[path = "../../capsule_net_nym/src/gateway_client/ws/frame/read.rs"]
pub mod read;
#[path = "../../capsule_net_nym/src/gateway_client/ws/frame/types.rs"]
pub mod types;
