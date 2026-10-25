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

//! The devices still able to reach all of RAM, the kernel included, on VT-d:
//! none. Bring-up puts every enumerated device in the blocked domain or in a
//! domain holding only its reserved memory (K2), never an identity map, and
//! a capsule's claim moves it into that capsule's domain. The posture line
//! asks every backend the same question, so the answer is said, not assumed.

/// How many devices translate through an identity map: always none.
pub fn identity_devices() -> usize {
    0
}
