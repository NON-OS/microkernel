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

//! `http` and `git clone` run as stepped jobs. Each tick the job asks the
//! clock about the far end: a direct socket's quiet ends a response, a quiet
//! through an anonymity network is a stall and fails it, the total bound
//! ends anything, and a handshake is never ended by quiet or a close. A
//! clone's git steps leave their one request in an `Ask` and get its answer
//! back once, never an answer meant for another request.

mod ask;
mod wait;
