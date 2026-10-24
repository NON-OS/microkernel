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

//! Each kind of record entry: its body read exactly, out-of-range values
//! refused, and kinds no version defines refused.

use nsh_wire::{EntryBody, Reason, WireError};

fn body(kind: u8, b: &[u8]) -> Result<EntryBody<'_>, WireError> {
    EntryBody::parse(kind, b)
}

#[test]
fn each_kind_reads_its_body() {
    let d64 = [0xDD; 64];
    let out = [&9u64.to_le_bytes()[..], &[0x0F; 32]].concat();
    assert_eq!(body(2, &out), Ok(EntryBody::Output { bytes: 9, digest: &[0x0F; 32] }));
    let call = [&0x4D43_4754u32.to_le_bytes()[..], &[5], b"Admin"].concat();
    assert_eq!(body(3, &call), Ok(EntryBody::RefusedCall { tag: 0x4D43_4754, cap: b"Admin" }));
    let granted = [&d64[..], &77u64.to_le_bytes(), &[1, 3], b"Dma"].concat();
    assert!(matches!(body(4, &granted), Ok(EntryBody::EscGranted { expires_ms: 77, .. })));
    let refused = [&33u16.to_le_bytes()[..], &d64].concat();
    let want = EntryBody::EscRefused { reason: Reason::EscApproval, grant: &d64 };
    assert_eq!(body(5, &refused), Ok(want));
    assert_eq!(
        body(6, &[&d64[..], &[2]].concat()),
        Ok(EntryBody::EscEnded { grant: &d64, why: 2 })
    );
    assert_eq!(body(8, &[0xC0; 32]), Ok(EntryBody::CommandDigest(&[0xC0; 32])));
}

#[test]
fn out_of_range_values_are_refused() {
    let d64 = [0xDD; 64];
    assert_eq!(body(3, &[&[0; 4][..], &[5], b"admin"].concat()), Err(WireError::Value));
    assert_eq!(body(4, &[&d64[..], &[0; 8], &[0]].concat()), Err(WireError::Value));
    assert_eq!(body(5, &[&0u16.to_le_bytes()[..], &d64].concat()), Err(WireError::Value));
    assert_eq!(body(5, &[&999u16.to_le_bytes()[..], &d64].concat()), Err(WireError::Value));
    assert_eq!(body(5, &[&1u16.to_le_bytes()[..], &d64].concat()), Err(WireError::Value));
    for why in [0, 4] {
        assert_eq!(body(6, &[&d64[..], &[why]].concat()), Err(WireError::Value), "{why}");
    }
    for kind in [0, 10, 255] {
        assert_eq!(body(kind, &[0; 4]), Err(WireError::Value), "{kind}");
    }
}

#[test]
fn each_fixed_body_is_exact() {
    assert_eq!(body(7, &[0; 3]), Err(WireError::Short));
    assert_eq!(body(7, &[0; 5]), Err(WireError::Trailing));
    assert_eq!(body(8, &[0; 33]), Err(WireError::Trailing));
    assert_eq!(body(2, &[0; 39]), Err(WireError::Short));
}
