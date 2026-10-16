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

//! The Accessibility page: each display's colour filter, high contrast and
//! magnifier, as the compositor holds them. Settings is their only client,
//! through the shell, which alone may hand them to the compositor.

pub mod knobs;
pub mod page;
pub mod shell;
pub mod text;
pub mod wire;

pub use knobs::AccessPage;
