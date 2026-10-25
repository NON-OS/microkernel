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

//! Whether a family's app is noted to slot 0 of the kernel's seal. A family
//! never noted is dropped at its end, so every proved app
//! is noted (I-C6). The capsule's own BusyBox has an app_id no verifier
//! takes, all zero, and is not: its family is dropped as before.

pub fn noted(app_id: &[u8; 32]) -> bool {
    *app_id != [0; 32]
}
