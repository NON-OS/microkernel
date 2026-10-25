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

//! A session record, `nsh-record/1`: a header, entries given as raw
//! (seq, time, kind, body) so a test can break any one, and a footer.

pub struct EntryB {
    pub seq: u32,
    pub at: u64,
    pub kind: u8,
    pub body: Vec<u8>,
}

pub struct RecordB {
    pub start: u64,
    pub caps: Vec<Vec<u8>>,
    pub entries: Vec<EntryB>,
    pub count: Option<u32>,
    pub end: u64,
    pub end_reason: u8,
}

/// A command entry, then an exit entry.
pub fn two_entries() -> Vec<EntryB> {
    let mut exit = Vec::new();
    exit.extend_from_slice(&0i32.to_le_bytes());
    vec![
        EntryB { seq: 0, at: 1_100, kind: 1, body: b"ls /var/log".to_vec() },
        EntryB { seq: 1, at: 1_200, kind: 7, body: exit },
    ]
}

impl Default for RecordB {
    fn default() -> Self {
        RecordB {
            start: 1_000,
            caps: vec![b"CoreExec".to_vec(), b"IPC".to_vec()],
            entries: two_entries(),
            count: None,
            end: 1_300,
            end_reason: 1,
        }
    }
}
