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

//! The clock the directory backoff measures against, and how long
//! each failure holds it off.

use core::sync::atomic::AtomicU64;

/// Uptime in milliseconds before the next attempt may run.
pub(crate) static NEXT_TRY_MS: AtomicU64 = AtomicU64::new(0);

/// How long to wait after the first failure, doubling from there.
pub(crate) static BACKOFF_MS: AtomicU64 = AtomicU64::new(FIRST_BACKOFF_MS);

/// A first wait long enough that a boot-time failure does not retry inside
/// the same second, short enough that a transient one costs nothing.
pub(crate) const FIRST_BACKOFF_MS: u64 = 500;

/// Ceiling on the wait.
pub(crate) const MAX_BACKOFF_MS: u64 = 30_000;

/// Uptime as an unsigned millisecond count.
pub(crate) fn now_ms() -> u64 {
    u64::try_from(nonos_libc::mk_uptime_ms()).unwrap_or(0)
}
