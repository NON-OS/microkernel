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

//! Timing one operation, many times.

use super::overhead::Overhead;
use super::summary::Summary;
use super::sort::sort;
use super::tsc::read_serialised;

/// Time `body` once per slot in `buf`, then reduce.
///
/// The caller owns the buffer because this crate allocates nothing and because
/// the sample count is the caller's decision: a syscall wants thousands, a
/// capsule spawn wants a handful, and burying that in a constant here would
/// make one of them wrong.
///
/// Warm-up runs are deliberately not offered. The first iterations of anything
/// on this machine are the ones that hit a cold cache and an unwarmed branch
/// predictor, and quietly dropping them is how a benchmark reports a system
/// nobody runs. They land in the tail, which is where they belong.
pub fn measure<F: FnMut()>(buf: &mut [u64], overhead: Overhead, mut body: F) -> Summary {
    for slot in buf.iter_mut() {
        let a = read_serialised();
        body();
        let b = read_serialised();
        *slot = overhead.correct(b.saturating_sub(a));
    }
    sort(buf);
    Summary::from_sorted(buf, overhead.cycles)
}
