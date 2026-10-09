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

//! A nox1 address, 2,009 letters, is the largest thing NONOS draws as a QR:
//! both the phone app's address and shield-core's pinned one fit, at
//! version 38 for error-correction level M. (Each was read back to its exact
//! text by an independent decoder, zxing-cpp, when versions 11 to 40 came in.)

const PHONE: &str = include_str!("../../wallet_proofs/fixtures/nox1-phone.txt");
const PINNED: &str = include_str!("../../wallet_proofs/fixtures/nox1-pinned.txt");

#[test]
fn a_nox1_address_fits_at_level_m() {
    for addr in [PHONE, PINNED] {
        let code = nonos_qr::encode(addr.as_bytes(), nonos_qr::Ecc::Medium).expect("fits");
        assert_eq!(code.size, 17 + 4 * 38);
    }
}

#[test]
fn past_version_40_nothing_is_drawn() {
    // 2,953 bytes is version 40's byte-mode capacity at level L.
    assert!(nonos_qr::encode(&[b'a'; 2953], nonos_qr::Ecc::Low).is_some());
    assert!(nonos_qr::encode(&[b'a'; 2954], nonos_qr::Ecc::Low).is_none());
}
