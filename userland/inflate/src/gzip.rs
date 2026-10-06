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

//! gzip, including the concatenated members RFC 1952 allows.

use alloc::vec::Vec;

use super::gzip_member::member;
use super::tables::MAX_OUT;
use super::types::{End, Inflated};

/// Enough for a distribution index in several parts.
pub(super) const MAX_MEMBERS: usize = 64;

/// Every member, concatenated, or `None` unless all of them decode and
/// check. Bytes after a verified member that do not start another member
/// are trailing garbage, and end the stream.
pub fn gunzip(data: &[u8]) -> Option<Vec<u8>> {
    gunzip_partial(data, MAX_OUT).complete()
}

/// Every member, concatenated, under the caller's bound on the whole output
/// in place of `MAX_OUT`; `None` when the output would pass it.
pub fn gunzip_within(data: &[u8], limit: usize) -> Option<Vec<u8>> {
    gunzip_partial(data, limit).complete()
}

/// Decodes members until the input ends, a member fails, or the output
/// reaches `cap`. A corrupt member after the first is trailing garbage.
pub fn gunzip_partial(data: &[u8], cap: usize) -> Inflated {
    let mut all = Inflated { out: Vec::new(), end: End::Complete, used: 0 };
    for n in 0..MAX_MEMBERS {
        let rest = &data[all.used..];
        if n > 0 && rest.is_empty() {
            return all;
        }
        let m = member(rest, cap - all.out.len());
        if n > 0 && m.end == End::Corrupt && m.out.is_empty() {
            return all;
        }
        if all.out.is_empty() {
            all.out = m.out;
        } else {
            all.out.extend_from_slice(&m.out);
        }
        all.used += m.used;
        if m.end != End::Complete {
            all.end = m.end;
            return all;
        }
    }
    if all.used != data.len() {
        all.end = End::Corrupt;
    }
    all
}
