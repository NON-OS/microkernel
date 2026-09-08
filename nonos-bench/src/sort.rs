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

//! Ordering a run before it is reduced.

/// Insertion sort. The buffers here are thousands of entries at most, it needs
/// no scratch allocation, and it is short enough to read in one sitting, which
/// matters more than an asymptote for a routine that runs after the timing has
/// already finished.
pub fn sort(buf: &mut [u64]) {
    for i in 1..buf.len() {
        let v = buf[i];
        let mut j = i;
        while j > 0 && buf[j - 1] > v {
            buf[j] = buf[j - 1];
            j -= 1;
        }
        buf[j] = v;
    }
}
