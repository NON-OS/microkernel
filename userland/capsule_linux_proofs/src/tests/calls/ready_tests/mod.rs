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

//! poll and select take their counts as Linux reads them and refuse what it
//! refuses, select names a closed descriptor EBADF and narrows its sets
//! without reading past them, and epoll reports level and edge as Linux
//! does, allows EPOLLEXCLUSIVE where it does, and refuses a loop of epolls.

mod epoll;
mod nest;
mod select;
mod web;
