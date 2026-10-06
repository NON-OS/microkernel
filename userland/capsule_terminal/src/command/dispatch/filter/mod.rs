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

pub mod count;
pub mod input;
mod text;

use alloc::vec;
use alloc::vec::Vec;

// Apply one pipe-stage filter to the previous stage's output lines.
pub(crate) fn apply(seg: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let args = seg.get(1..).unwrap_or(&[]);
    match seg.first().copied().unwrap_or(b"") {
        b"grep" => text::grep(args, input),
        b"sort" => text::sort(args, input),
        b"uniq" => text::uniq(args, input),
        b"tac" => text::tac(input),
        b"rev" => text::rev(input),
        b"cut" => text::cut(args, input),
        b"nl" => text::nl(input),
        b"wc" => count::wc(args, input),
        b"head" => count::head(args, input),
        b"tail" => count::tail(args, input),
        other => {
            let mut msg = Vec::new();
            msg.extend_from_slice(b"pipe: unknown filter ");
            msg.extend_from_slice(other);
            vec![msg]
        }
    }
}
