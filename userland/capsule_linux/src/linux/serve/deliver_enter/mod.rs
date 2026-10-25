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

//! Entering a handler: the frame on the thread's stack, or at the top of its
//! alternate stack when the handler asked for SA_ONSTACK and the thread is not
//! already on it; the mask while the handler runs is the thread's own, the
//! handler's sa_mask, and the signal itself unless SA_NODEFER.

mod enter;
mod handler;

pub use enter::enter;
