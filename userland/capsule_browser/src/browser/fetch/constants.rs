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

pub const MAX_BODY: usize = 4 * 1024 * 1024;
pub const MAX_TLS_FLIGHT: usize = 512 * 1024;
pub const MAX_REDIRECTS: u8 = 5;
pub const MAX_RETRIES: u8 = 2;
/* The longest one drain reads for; it stops sooner at the first empty read. */
pub const DRAIN_MS: i64 = 25;
/* The longest one tick steps fetches for before letting the window draw. */
pub const TICK_MS: i64 = 30;
