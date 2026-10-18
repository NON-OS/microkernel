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

//! E-65: HCI events from a controller whose firmware is not ours. The host's
//! event classifier `answer` is driven over seeded event frames — any code,
//! any stated length, truncated and overlong (`sweep`). It never panics; a
//! length that disagrees with the frame is refused; an unknown event
//! completes nothing; and a Command Complete or Status completes only the
//! command actually waited for, never one never sent.

mod kit;
mod sweep;
