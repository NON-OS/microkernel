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
//! net.nym's send queue and the step that releases it, as the capsule
//! compiles them; the gateway write and the cover packet are the tests'.

#[path = "../../../capsule_net_nym/src/server/outbound/queue.rs"]
pub mod queue;

#[path = "../../../capsule_net_nym/src/server/outbound/step.rs"]
pub mod step;
