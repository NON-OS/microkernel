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

//! The stream wire formats, included from the real capsule source.

#[path = "../../../capsule_net_anon/src/stream/begin.rs"]
pub mod begin;

#[path = "../../../capsule_net_anon/src/stream/sendme.rs"]
pub mod sendme;

#[path = "../../../capsule_net_anon/src/stream/end.rs"]
pub mod end;

#[path = "../../../capsule_net_anon/src/stream/connected.rs"]
pub mod connected;

#[path = "../../../capsule_net_anon/src/stream/data.rs"]
pub mod data;

#[path = "../../../capsule_net_anon/src/stream/ids.rs"]
pub mod ids;

#[path = "../../../capsule_net_anon/src/stream/stage.rs"]
pub mod stage;

#[path = "../../../capsule_net_anon/src/stream/table.rs"]
pub mod table;

#[path = "../../../capsule_net_anon/src/stream/grant.rs"]
pub mod grant;

pub use stage::StreamStage;
pub use table::Stream;
