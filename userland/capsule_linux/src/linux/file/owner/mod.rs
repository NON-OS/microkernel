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

/*
 * Owners and times: what the store does not record.
 *
 * The store keeps bytes under names and nothing else, so every file reports
 * uid 0, gid 0 and time zero, and the guest runs as uid 0. A change that
 * would leave that true is answered; one that would need the store to keep
 * something it cannot is refused by name, never reported done.
 */

mod chown;
mod stamp;
mod times;

pub(super) use chown::refused;
pub use chown::{fchown_ids, fchownat};
pub use times::{utimensat, utimes};
