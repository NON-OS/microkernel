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

//! A guest's selections, the clipboard and the primary one: kept as the
//! guest's own, offered to every device it holds, and read by its own
//! clients. Text alone is carried to and from the desktop clipboard, and
//! only for the window with focus (desk.rs). The pure parts, which the
//! host proofs mount, are gone.rs, mime.rs, offer.rs, set.rs, sources.rs,
//! state.rs and which.rs.

pub(super) mod ask;
pub(super) mod desk;
pub(super) mod desk_read;
pub(super) mod focus;
pub(super) mod gone;
pub(super) mod mime;
pub(super) mod offer;
pub(super) mod set;
pub(super) mod sources;
pub(super) mod state;
pub(super) mod take;
pub(super) mod which;

use super::object::{Object, Objects};
use super::ops::{data, select};
use super::out::Event;

pub use state::Clip;
