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

//! What the window knows: the chats, the open one, the compose line, where
//! the keys go, and the commands waiting for the core. Pure, so
//! simplex_ui_proofs holds it on the host.

pub mod apply;
pub mod chat;
pub mod command;
pub mod compose;
pub mod copy;
pub mod focus;
pub mod items;
pub mod key;

pub use focus::Key;
pub mod nav;
pub mod read;
pub mod says;
pub mod start;
pub mod start_name;
pub mod start_phase;
pub mod state;
pub mod take;
