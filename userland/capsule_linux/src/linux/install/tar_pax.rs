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

//! Pax extended headers: `length key=value\n` records that override the
//! next entry's fields. Only the two that change where a file lands are
//! taken; timestamps and checksums are recorded by nothing here.

use alloc::vec::Vec;

#[derive(Default)]
pub struct Overrides {
    pub path: Option<Vec<u8>>,
    pub link: Option<Vec<u8>>,
}

pub fn read(body: &[u8], into: &mut Overrides) {
    let mut at = 0usize;
    while at < body.len() {
        // The length counts the whole record, its own digits included.
        let Some(space) = body[at..].iter().position(|b| *b == b' ') else {
            return;
        };
        let Some(len) = decimal(&body[at..at + space]) else {
            return;
        };
        let Some(end) = at.checked_add(len).filter(|e| *e <= body.len() && len > space + 1) else {
            return;
        };
        let record = &body[at + space + 1..end - 1];
        if let Some(eq) = record.iter().position(|b| *b == b'=') {
            let value = record[eq + 1..].to_vec();
            match &record[..eq] {
                b"path" => into.path = Some(value),
                b"linkpath" => into.link = Some(value),
                _ => {}
            }
        }
        at = end;
    }
}

fn decimal(text: &[u8]) -> Option<usize> {
    if text.is_empty() {
        return None;
    }
    text.iter().try_fold(0usize, |v, b| match b {
        b'0'..=b'9' => v.checked_mul(10)?.checked_add((b - b'0') as usize),
        _ => None,
    })
}
