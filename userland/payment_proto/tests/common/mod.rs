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

//! What every body test checks: no cut and no tail reads.

pub fn refused_cut_and_tail<T>(b: &[u8], parse: impl Fn(&[u8]) -> Option<T>) {
    for n in 0..b.len() {
        assert!(parse(&b[..n]).is_none(), "{n} of {} bytes", b.len());
    }
    let mut long = b.to_vec();
    long.push(0);
    assert!(parse(&long).is_none());
}
