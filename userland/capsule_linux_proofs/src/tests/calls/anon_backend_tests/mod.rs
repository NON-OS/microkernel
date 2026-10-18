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

//! A net.anon stream is closed exactly once: on the last close of the
//! socket, on shutdown of both sides, or when its entry is freed, whichever
//! comes first, and never again after. A mixnet stream is let go as it
//! always was. Driven through the socket table as it ships, with the two
//! close calls recorded instead of sent.

mod adopt;
mod close;
mod ids;
mod order;
mod share;
mod sides;
