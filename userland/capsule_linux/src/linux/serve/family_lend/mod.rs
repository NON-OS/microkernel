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

//! Lending the family's pipes, eventfd counters and timerfd timers to the
//! guest being answered.
//!
//! All three are the family's, since a fork leaves their descriptors in more
//! than one process. The guest being answered holds them for that one answer,
//! with a note of which ends are still open anywhere in the family: end of
//! file, a broken pipe and a hang-up are all decided by that note.

mod lend;
mod note;
