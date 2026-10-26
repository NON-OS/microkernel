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

//! An isolation witness wiped in place: its histogram and its salt.

use isolation_stark::Witness;

pub fn clear(w: &mut Witness) {
    // SAFETY: valid, aligned fields of `w`, borrowed mutably.
    w.hist.iter_mut().for_each(|h| unsafe { core::ptr::write_volatile(h, 0) });
    w.salt.iter_mut().for_each(|x| unsafe { core::ptr::write_volatile(x, 0) });
}
