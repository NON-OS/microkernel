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

//! Serving the host wire: the state a request works on, each op's handler,
//! and the dispatch between them. Handlers build their reply in the buffer
//! and return its length; only the capsule's loop sends it, so the host
//! proofs run every handler against a modelled controller.

pub mod context;
pub mod dispatch;
mod handlers;
pub mod reply;
