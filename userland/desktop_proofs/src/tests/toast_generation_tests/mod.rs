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

//! The toasts are part of the chrome's frame, and the shell repaints that
//! frame when the queue's generation moved past the one it was painted with
//! (`render/toasts.rs` sync_toast_layer). Every change to what the panel
//! shows must move the generation, and nothing else may, or a toast stays
//! on screen after it went, or the chrome is repainted every clock tick.

mod chrome;
mod full;
mod painted;
