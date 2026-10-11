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

//! The display server walks a batch of requests where it lies and cuts
//! what it served from the queue once. Walking takes the queue by shared
//! reference, so nothing in it can cut the queue a message at a time, which
//! moved every byte behind each message. Held here: every whole message is
//! handed on once and in order, a refused one is still walked past, and the
//! walk stops at the first message not yet whole.

mod front;
mod order;
