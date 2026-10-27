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

//! A guest's stream that did not open: which answers are waited out and for
//! how long (the 45 s an install allows), what the guest's connect is told,
//! and the one line the person reads, which names the network and never
//! the host or the port. And one closed with bytes still queued: they go,
//! in order and once, before it ends, within its bound.

mod errno;
mod flush;
mod flush_bound;
mod line;
mod proxy;
mod retry;
mod window;
