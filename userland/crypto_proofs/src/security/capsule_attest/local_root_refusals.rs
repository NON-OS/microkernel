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

//! What a local trailer must not verify for.

use super::local_root_tests::{minted, ELF};
use super::against_pedersen::verify;

#[test]
fn it_does_not_verify_for_another_image_or_other_capabilities() {
    let (root, trailer) = minted(7, ELF, 0);
    assert!(verify(&trailer, b"\x7fELF something else", 0, &root).is_err());
    assert!(verify(&trailer, ELF, 1 << 33, &root).is_err());
    assert!(verify(&trailer, ELF, 0x8, &root).is_err());
}

#[test]
fn another_machine_root_refuses_it() {
    let (_, trailer) = minted(7, ELF, 0);
    let (other_root, _) = minted(9, ELF, 0);
    assert!(verify(&trailer, ELF, 0, &other_root).is_err());
}

#[test]
fn tampering_or_guessing_the_secret_fails() {
    let (root, trailer) = minted(7, ELF, 0);
    for i in [8, 40, 72, 104, 137] {
        let mut t = trailer.clone();
        t[i] ^= 1;
        assert!(verify(&t, ELF, 0, &root).is_err(), "flipped byte {i}");
    }
    let (_, guessed) = minted(8, ELF, 0);
    assert!(verify(&guessed, ELF, 0, &root).is_err());
}
