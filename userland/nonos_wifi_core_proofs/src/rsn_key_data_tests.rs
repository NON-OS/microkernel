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

//! The RSNXE's H2E bit, and the key data walk: it finds the RSNE, RSNXE,
//! GTK and IGTK and stops at padding.

use nonos_wifi_core::eapol::kde::parse_key_data;
use nonos_wifi_core::rsn::rsnxe::advertises_h2e;

#[test]
fn the_rsnxe_h2e_bit_is_read() {
    assert!(advertises_h2e(&[0x20]));
    assert!(!advertises_h2e(&[0x00]));
    assert!(!advertises_h2e(&[]));
}

#[test]
fn key_data_is_walked_to_its_padding() {
    let mut kd = vec![48, 2, 1, 0]; // a minimal RSNE
    kd.extend_from_slice(&[244, 1, 0x20]); // RSNXE
    kd.extend_from_slice(&[0xDD, 22, 0x00, 0x0f, 0xac, 0x01, 0x06, 0x00]);
    kd.extend_from_slice(&[0x5A; 16]);
    kd.extend_from_slice(&[0xDD, 0, 0, 0]); // padding
    let p = parse_key_data(&kd).unwrap();
    assert_eq!(p.rsne, Some(&[48u8, 2, 1, 0][..]));
    assert_eq!(p.rsnxe, Some(&[244u8, 1, 0x20][..]));
    let g = p.gtk.unwrap();
    assert_eq!((g.key_id, g.key), (2, &[0x5Au8; 16][..]), "only the index bits of 0x06");
    assert!(p.igtk.is_none());
    // A KDE that runs past the end is refused; padding that does is not.
    assert!(parse_key_data(&[0xDD, 10, 0x00, 0x0f, 0xac, 0x01]).is_none());
    assert!(parse_key_data(&[0xDD, 0x00, 0x00]).is_some());
    assert!(parse_key_data(&[0xDD, 22, 0x00, 0x0f, 0xac, 0x01, 1]).is_none(), "a cut GTK KDE");
}
