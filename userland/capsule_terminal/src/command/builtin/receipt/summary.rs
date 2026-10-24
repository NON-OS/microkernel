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

//! The one count that matters, stated rather than left to be counted.

use alloc::vec::Vec;
use core::slice::ChunksExact;

use super::row::{caps_of, network_bit};
use crate::command::output::Output;
use crate::term::util::format_u64;

/// "N of M capsules <said>.", the shape every count line takes.
pub(crate) fn count_line(n: usize, of: usize, said: &[u8]) -> Vec<u8> {
    let mut line: Vec<u8> = Vec::with_capacity(96);
    let mut num = [0u8; 12];
    let k = format_u64(n as u64, &mut num);
    line.extend_from_slice(&num[..k]);
    line.extend_from_slice(b" of ");
    let k = format_u64(of as u64, &mut num);
    line.extend_from_slice(&num[..k]);
    line.extend_from_slice(b" capsules ");
    line.extend_from_slice(said);
    line.push(b'.');
    line
}

pub(super) fn summary(out: &mut Output<'_>, entries: ChunksExact<'_, u8>) {
    let total = entries.len();
    let networked = entries.filter(|e| caps_of(e) & network_bit() != 0).count();
    let mut line = count_line(networked, total, b"can reach the network");
    line.extend_from_slice(b" Check off-machine with `receipt --hex`.");
    out.writeln(b"");
    out.writeln(&line);
}
