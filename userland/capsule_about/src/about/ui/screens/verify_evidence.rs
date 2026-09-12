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

//! The count behind a verdict, as one short cell.

use crate::about::data::verify::{Check, Verdict};
use crate::about::format::u64_decimal;

use super::append::put;

// The count that settled a check, phrased as what was looked for rather than as
// a score: "0 of 44" says both that nothing was found and how wide the search
// was, which a bare tick never does.
pub(super) fn evidence<'a>(c: &Check, buf: &'a mut [u8; 32]) -> &'a [u8] {
    if c.verdict == Verdict::Unknown {
        return b"not established";
    }
    let mut found = [0u8; 20];
    let mut of = [0u8; 20];
    let found = u64_decimal(c.found as u64, &mut found);
    let of = u64_decimal(c.of as u64, &mut of);
    let mut n = 0;
    n += put(&mut buf[n..], found);
    n += put(&mut buf[n..], b" of ");
    n += put(&mut buf[n..], of);
    &buf[..n]
}
