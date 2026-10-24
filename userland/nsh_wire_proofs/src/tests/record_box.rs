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

//! The `BOX` entry: BOX REPORT v1's fields, read exactly, with the kinds in
//! that report's fixed order (docs/design/nsh-v1-record.md, "The `BOX`
//! entry").

use nsh_wire::record::{BOX_BODY, BOX_KINDS};
use nsh_wire::{BoxRun, EntryBody, Preset, WireError};

pub(super) fn layout(preset: u8, started: u8, exit: i32, counts: [u64; 9]) -> Vec<u8> {
    let mut b = vec![preset];
    b.extend_from_slice(&[0x58; 32]);
    b.push(started);
    b.extend_from_slice(&exit.to_le_bytes());
    counts.iter().for_each(|c| b.extend_from_slice(&c.to_le_bytes()));
    b
}

pub(super) fn run(b: &[u8]) -> Result<BoxRun<'_>, WireError> {
    match EntryBody::parse(9, b)? {
        EntryBody::Box(run) => Ok(run),
        other => panic!("read as {other:?}"),
    }
}

#[test]
fn the_described_example_reads_field_for_field() {
    // The example line in the report's description: strict, write=1
    // network=1, exit 1.
    let b = layout(0, 1, 1, [1, 1, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(b.len(), BOX_BODY);
    assert_eq!(BOX_BODY, 110);
    let r = run(&b).unwrap();
    assert_eq!((r.preset, r.policy, r.started, r.exit), (Preset::Strict, &[0x58; 32], true, 1));
    assert_eq!(r.counts, [1, 1, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(r.refused(), 2);
}

#[test]
fn each_count_lands_at_its_kind() {
    let want = ["write", "network", "exec", "jit", "display", "models", "call", "barred"];
    assert_eq!(BOX_KINDS[..8], want);
    assert_eq!(BOX_KINDS[8], "unserved");
    for i in 0..BOX_KINDS.len() {
        let mut counts = [0; 9];
        counts[i] = 0x0102_0304_0506_0700 + i as u64;
        let b = layout(2, 1, -9, counts);
        let r = run(&b).unwrap();
        assert_eq!((r.counts, r.preset, r.exit), (counts, Preset::Allow, -9), "{i}");
    }
    let b = layout(1, 0, 0, [0; 9]);
    let r = run(&b).unwrap();
    assert_eq!((r.preset, r.started), (Preset::Offline, false));
}
