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

//! The capability-name lists that AUTH_OK, the grant and the record share.

use super::auth_ok::list;
use nsh_wire::{CapNames, Reader, WireError, MAX_CAPS};

fn read(bytes: &[u8]) -> Result<Vec<Vec<u8>>, WireError> {
    let mut r = Reader::new(bytes);
    let caps = CapNames::read(&mut r)?;
    r.finish()?;
    Ok(caps.iter().map(<[u8]>::to_vec).collect())
}

#[test]
fn a_name_is_an_uppercase_letter_then_letters_and_digits() {
    assert!(read(&list(&[b"Dma", b"GraphicsSurfaceMap", b"X9"])).is_ok());
    for bad in [&b"dma"[..], b"9Dma", b"Dm a", b"Dm-a", b"Dm\x00", b"\xC3\x89"] {
        assert_eq!(read(&list(&[bad])), Err(WireError::Value), "{bad:?}");
    }
    assert_eq!(read(&list(&[&[b'A'; 33]])), Err(WireError::Oversize));
    assert!(read(&list(&[&[b'A'; 32]])).is_ok());
    assert_eq!(read(&[1, 0]), Err(WireError::Value));
}

#[test]
fn a_list_names_each_capability_once_and_at_most_thirty_seven() {
    assert_eq!(read(&list(&[b"IPC", b"Dma", b"IPC"])), Err(WireError::Value));
    let names: Vec<Vec<u8>> = (0..38).map(|i| format!("C{i}").into_bytes()).collect();
    let refs: Vec<&[u8]> = names.iter().map(Vec::as_slice).collect();
    assert_eq!(read(&list(&refs[..MAX_CAPS])).map(|v| v.len()), Ok(37));
    assert_eq!(read(&list(&refs)), Err(WireError::Oversize));
    assert_eq!(read(&[0]), Ok(vec![]));
}
