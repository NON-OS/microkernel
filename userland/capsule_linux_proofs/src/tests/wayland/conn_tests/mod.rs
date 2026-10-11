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

//! The display connection's two queues are this capsule's memory. A client
//! that sends requests and never reads their answers is held as a socket
//! holds a writer whose reader has stopped: the server stops serving once
//! MOST_UNREAD waits for the client, the requests wait in turn, and a write
//! past MOST_UNSENT takes nothing. Neither queue grows past its bound plus
//! one answer, however long the client keeps at it.

mod bounds;
mod held;
